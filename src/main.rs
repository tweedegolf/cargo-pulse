use chrono::{self, Duration};
use crates_io_api::{CrateResponse, Error, SyncClient};

fn seniority(pkg: &CrateResponse) -> Duration {
    chrono::offset::Utc::now() - pkg.crate_data.created_at
}

fn staleness(pkg: &CrateResponse) -> Duration {
    let now = chrono::offset::Utc::now();
    chrono::offset::Utc::now() - pkg.crate_data.updated_at
}

fn frequency(pkg: &CrateResponse, period: Duration) -> f64 {
    let average_age = seniority(pkg) / pkg.versions.len() as i32;
    period.num_milliseconds() as f64 / average_age.num_milliseconds() as f64
}

fn main() {
    let crates_io = SyncClient::new(
        "cargo pulse (info@tweedegolf.com)",
        std::time::Duration::from_millis(1000),
    )
    .unwrap();

    let pkg = crates_io.get_crate("serde").unwrap();

    println!("{:?}", seniority(&pkg));
    println!("{:?}", staleness(&pkg));
    println!("{:?}", frequency(&pkg, Duration::days(100)));
    println!("{:?}", pkg.crate_data.repository);
}
