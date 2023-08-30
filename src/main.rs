use crates_io_api::{SyncClient, Error, CrateResponse};
use chrono;

trait Vitality {
    fn seniority(&self) -> chrono::Duration;
    fn staleness(&self) -> chrono::Duration;
    fn average_age(&self) -> chrono::Duration;
}


impl Vitality for CrateResponse {
    fn seniority(&self) -> chrono::Duration {
        chrono::offset::Utc::now() - self.crate_data.created_at
    }

    fn staleness(&self) -> chrono::Duration {
        let now = chrono::offset::Utc::now();
        chrono::offset::Utc::now() - self.crate_data.updated_at
    }

    fn average_age(&self) -> chrono::Duration {
        self.seniority() / self.versions.len() as i32
    }
}

fn main() {
    let crates_io = SyncClient::new(
         "cargo pulse (info@tweedegolf.com)",
         std::time::Duration::from_millis(1000),
    ).unwrap();

    let pkg = crates_io.get_crate("serde").unwrap();

    println!("{:?}", pkg.seniority());
    println!("{:?}", pkg.staleness());
    println!("{:?}", pkg.average_age());
    println!("{:?}", pkg.crate_data.repository);
}
