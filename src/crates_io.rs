use chrono::{self, Duration};
use crates_io_api::SyncClient;

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

pub fn get_crate_info(crate_name: &str) -> CrateVitality {
    let crates_io = SyncClient::new(
        "cargo pulse (info@tweedegolf.com)",
        std::time::Duration::from_millis(1000),
    )
    .unwrap();

    let pkg = crates_io.get_crate(crate_name).unwrap();

    let now = chrono::offset::Utc::now();

    let full_age = now - pkg.crate_data.created_at;
    let staleness = now - pkg.crate_data.updated_at;
    let recent_downloads = pkg.crate_data.recent_downloads.unwrap_or_default();

    let update_frequency = Duration::days(90).num_milliseconds() as f64 * pkg.versions.len() as f64
        / full_age.num_milliseconds() as f64;

    let gh = pkg
        .crate_data
        .repository
        .as_deref()
        .and_then(crate::github::fetch_github_data);

    let stargazers = gh.as_ref().map(|info| info.stargazers);
    let authors = gh.as_ref().map(|info| info.authors);

    let reverse_dependencies = crates_io.crate_reverse_dependency_count(crate_name).ok();

    CrateVitality {
        full_age,
        staleness,
        update_frequency,
        recent_downloads,
        reverse_dependencies,
        stargazers,
        authors,
    }
}
