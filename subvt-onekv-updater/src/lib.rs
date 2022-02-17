//! Updates the complete 1KV data for the network (only Polkadot and Kusama) on the database.

use async_trait::async_trait;
use lazy_static::lazy_static;
use log::{debug, error, info};
use subvt_config::Config;
use subvt_persistence::postgres::network::PostgreSQLNetworkStorage;
use subvt_service_common::Service;
use subvt_types::onekv::{OneKVCandidate, OneKVCandidateDetails, OneKVNominator};

lazy_static! {
    static ref CONFIG: Config = Config::default();
}

pub struct OneKVUpdater {
    http_client: reqwest::Client,
}

impl Default for OneKVUpdater {
    fn default() -> Self {
        let http_client: reqwest::Client = reqwest::Client::builder()
            .gzip(true)
            .brotli(true)
            .timeout(std::time::Duration::from_secs(
                CONFIG.onekv.request_timeout_seconds,
            ))
            .build()
            .unwrap();
        Self { http_client }
    }
}

impl OneKVUpdater {
    async fn update_candidates(&self, postgres: &PostgreSQLNetworkStorage) -> anyhow::Result<()> {
        info!("Fetch candidate list.");
        let response = self
            .http_client
            .get(&CONFIG.onekv.candidate_list_endpoint)
            .send()
            .await?;
        let candidates: Vec<OneKVCandidate> = response.json().await?;
        info!(
            "Fetched {} candidates. Fetch candidate details.",
            candidates.len()
        );
        // get details for each candidate
        for (index, candidate) in candidates.iter().enumerate() {
            let response_result = self
                .http_client
                .get(&format!(
                    "{}{}",
                    CONFIG.onekv.candidate_details_endpoint, candidate.stash_address
                ))
                .send()
                .await;
            let response = match response_result {
                Ok(response) => response,
                Err(error) => {
                    error!(
                        "Error while fetching details for candidate {}:{:?}",
                        candidate.stash_address, error
                    );
                    continue;
                }
            };

            let candidate_details_result: reqwest::Result<OneKVCandidateDetails> =
                response.json().await;
            let mut candidate_details = match candidate_details_result {
                Ok(candidate_details) => candidate_details,
                Err(error) => {
                    error!(
                        "Error while deserializing details JSON for candidate {}:{:?}",
                        candidate.stash_address, error
                    );
                    continue;
                }
            };
            candidate_details.score = candidate.score.clone();
            let save_result = postgres
                .save_onekv_candidate(
                    &candidate_details,
                    CONFIG.onekv.candidate_history_record_count as i64,
                )
                .await;
            match save_result {
                Ok(_) => {
                    debug!(
                        "Fetched and persisted candidate {} of {} :: {}.",
                        index + 1,
                        candidates.len(),
                        candidate.stash_address,
                    );
                }
                Err(error) => {
                    error!(
                        "Error while persisting details of candidate {}:{:?}",
                        candidate.stash_address, error
                    );
                }
            }
        }
        info!("1KV update completed.");
        Ok(())
    }
}

impl OneKVUpdater {
    async fn update_nominators(&self, postgres: &PostgreSQLNetworkStorage) -> anyhow::Result<()> {
        info!("Fetch nominator list.");
        let response = self
            .http_client
            .get(&CONFIG.onekv.nominator_list_endpoint)
            .send()
            .await?;
        let nominators: Vec<OneKVNominator> = response.json().await?;
        info!("Fetched {} nominators.", nominators.len());
        for (index, nominator) in nominators.iter().enumerate() {
            let save_result = postgres
                .save_onekv_nominator(
                    nominator,
                    CONFIG.onekv.candidate_history_record_count as i64,
                )
                .await;
            match save_result {
                Ok(_) => {
                    debug!(
                        "Persisted nominator {} of {} :: {}.",
                        index + 1,
                        nominators.len(),
                        nominator.address,
                    );
                }
                Err(error) => {
                    error!(
                        "Error while persisting nominator {}:{:?}",
                        nominator.address, error
                    );
                }
            }
        }
        Ok(())
    }
}

#[async_trait(?Send)]
impl Service for OneKVUpdater {
    async fn run(&'static self) -> anyhow::Result<()> {
        info!(
            "1KV updater has started with {} seconds refresh wait period.",
            CONFIG.onekv.refresh_seconds
        );
        let postgres =
            PostgreSQLNetworkStorage::new(&CONFIG, CONFIG.get_network_postgres_url()).await?;
        loop {
            info!("Update 1KV candidates.");
            if let Err(error) = self.update_candidates(&postgres).await {
                error!("1KV candidates update has failed: {:?}", error);
            }
            info!("Update 1KV nominators.");
            if let Err(error) = self.update_nominators(&postgres).await {
                error!("1KV nominators update has failed: {:?}", error);
            }
            info!("Sleep for {} seconds.", CONFIG.onekv.refresh_seconds);
            std::thread::sleep(std::time::Duration::from_secs(CONFIG.onekv.refresh_seconds));
        }
    }
}
