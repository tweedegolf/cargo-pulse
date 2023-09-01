use chrono::{self, Duration};
use crates_io_api::AsyncClient;

#[derive(Debug)]
pub struct CrateVitality {
    pub full_age: Duration,
    pub staleness: Duration,
    pub update_frequency: f64,
    pub recent_downloads: u64,
    pub reverse_dependencies: Option<u64>,
    pub stargazers: Option<u32>,
    pub authors: Option<u32>,
}

pub fn spawn() -> AsyncClient {
    AsyncClient::new(
        "cargo pulse (info@tweedegolf.com)",
        std::time::Duration::from_millis(1000),
    )
    .unwrap()
}

use async_trait::async_trait;

#[async_trait]
pub trait GetVitalSigns {
    async fn get_vital_signs(
        &mut self,
        crate_name: &str,
    ) -> Result<CrateVitality, crates_io_api::Error>;
}

#[async_trait]
impl GetVitalSigns for AsyncClient {
    async fn get_vital_signs(
        &mut self,
        crate_name: &str,
    ) -> Result<CrateVitality, crates_io_api::Error> {
        let pkg = self.get_crate(crate_name).await?;

        let now = chrono::offset::Utc::now();

        let full_age = now - pkg.crate_data.created_at;
        let staleness = now - pkg.crate_data.updated_at;
        let recent_downloads = pkg.crate_data.recent_downloads.unwrap_or_default();

        let update_frequency = Duration::days(90).num_milliseconds() as f64
            * pkg.versions.len() as f64
            / full_age.num_milliseconds() as f64;

        let gh = if let Some(uri) = pkg.crate_data.repository {
            Some(crate::github::fetch_github_data(&uri).await.unwrap())
        } else {
            None
        };

        let stargazers = gh.as_ref().map(|info| info.stargazers);
        let authors = gh.as_ref().map(|info| info.authors);

        let reverse_dependencies = self.crate_reverse_dependency_count(crate_name).await.ok();

        Ok(CrateVitality {
            full_age,
            staleness,
            update_frequency,
            recent_downloads,
            reverse_dependencies,
            stargazers,
            authors,
        })
    }
}
