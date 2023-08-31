use chrono::{self, Duration};
use crates_io_api::{CrateResponse, SyncClient};

fn seniority(pkg: &CrateResponse) -> Duration {
    chrono::offset::Utc::now() - pkg.crate_data.created_at
}

fn staleness(pkg: &CrateResponse) -> Duration {
    chrono::offset::Utc::now() - pkg.crate_data.updated_at
}

fn frequency(pkg: &CrateResponse, period: Duration) -> f64 {
    period.num_milliseconds() as f64 * pkg.versions.len() as f64
        / seniority(pkg).num_milliseconds() as f64
}

#[derive(Debug)]
struct GhStats {
    pub stargazers: u32,
    //pub watchers: u32,
    pub authors: u32,
}

fn fetch_github_data(url: &str) -> Option<GhStats> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let (owner, repo);
    {
        let mut iter = url.split('/').rev().take(2);
        repo = iter.next().unwrap();
        owner = iter.next().unwrap();
    }

    let gh_data =
        runtime.block_on(async { octocrab::instance().repos(owner, repo).get().await.unwrap() });

    let gh_contributors: octocrab::Page<octocrab::models::Author> = runtime.block_on(async {
        octocrab::instance()
            .get(format!("/repos/{owner}/{repo}/contributors"), None::<&()>)
            .await
            .unwrap()
    });

    Some(GhStats {
        stargazers: gh_data.stargazers_count.unwrap_or_default(),
        //watchers: gh_data.watchers_count.unwrap_or_default(),  // this data is incorrect in octocrab
        authors: gh_contributors.items.len() as u32,
    })
}

fn main() {
    let crates_io = SyncClient::new(
        "cargo pulse (info@tweedegolf.com)",
        std::time::Duration::from_millis(1000),
    )
    .unwrap();

    let crate_name = std::env::args().nth(1).unwrap();

    let pkg = crates_io.get_crate(&crate_name).unwrap();

    println!("[{crate_name}]");
    println!("full age {:?}", seniority(&pkg).num_days());
    println!("staleness {:?}", staleness(&pkg).num_days());
    println!("upd100 {:?}", frequency(&pkg, Duration::days(100)));
    println!("dlds {:?}", pkg.crate_data.downloads);
    println!(
        "{:?}",
        pkg.crate_data
            .repository
            .as_deref()
            .and_then(fetch_github_data)
    );
    println!(
        "# {:?}",
        crates_io
            .crate_reverse_dependency_count(&crate_name)
            .unwrap()
    );
}
