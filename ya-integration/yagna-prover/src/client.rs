use anyhow::anyhow;
use reqwest::Url;
use std::fs::File;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{self};
use zksync_crypto::proof::EncodedProofPlonk;
use zksync_prover::ApiClient;
use zksync_prover_utils::prover_data::ProverData;

#[derive(Debug, Clone)]
pub struct YagnaApiClient {
    finish: Arc<AtomicBool>,
}

impl YagnaApiClient {
    pub fn new(_base_url: &Url, _worker: &str, _req_server_timeout: time::Duration) -> Self {
        YagnaApiClient {
            finish: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl ApiClient for YagnaApiClient {
    fn block_to_prove(&self, _block_size: usize) -> Result<Option<(i64, i32)>, anyhow::Error> {
        // Download block info from disk.
        // Yagna Requestor should upload json file for us.
        Ok(Some((0, 0)))
    }

    fn working_on(&self, _job_id: i32) -> Result<(), anyhow::Error> {
        // This will be responsibility of Yagna Requestor.
        Ok(())
    }

    fn prover_data(&self, block: i64) -> Result<ProverData, anyhow::Error> {
        // Yagna Requestor will command ExeUnit to download block and place in our directories.
        let block_path = proves_info_dir().join("block_data.json");

        let json_file = File::open(&block_path).map_err(|e| {
            anyhow!(
                "Can't open block file [{}] to deserialize. Error: {}",
                &block_path.display(),
                e
            )
        })?;
        let prover_data: ProverData = serde_json::from_reader(json_file)
            .map_err(|e| anyhow!("Failed to deserialize block {}. Error: {}", block, e))?;

        Ok(prover_data)
    }

    fn publish(&self, block: i64, proof: EncodedProofPlonk) -> Result<(), anyhow::Error> {
        // Serialize proof and save on disk.
        // Yagna Requestor will download it from expected location and send to zksync server.
        let proof_path = proves_info_dir().join("proof_data.json");
        let file = File::open(&proof_path).map_err(|e| {
            anyhow!(
                "Can't open proof file [{}]. Error: {}",
                proof_path.display(),
                e
            )
        })?;

        serde_json::to_writer(file, &proof)
            .map_err(|e| anyhow!("Failed to serialize block {}. Error: {}", block, e))?;

        // We run only single proof. Yagna Requestor will run VM multiple times.
        // TODO: Implement it better on Requestor side.
        self.finish.store(true, Ordering::SeqCst);
        Ok(())
    }

    fn prover_stopped(&self, _prover_run_id: i32) -> Result<(), anyhow::Error> {
        // Yagna Reuestor will notify server, after prover will stop.
        Ok(())
    }

    fn register_prover(&self, _block_size: usize) -> Result<i32, anyhow::Error> {
        // Prover doesn't see external world so it can be anything.
        Ok(32)
    }
}

pub fn proves_info_dir() -> PathBuf {
    PathBuf::from("/data")
}
