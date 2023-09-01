use cargo_metadata::MetadataCommand;

mod crates_io;
use crates_io::GetVitalSigns;
mod github;

async fn display_vital_signs<'a>(crates: impl IntoIterator<Item = &'a str>) {
    let mut client = crates_io::spawn();
    for name in crates {
        println!("[{name}] {:?}", client.get_vital_signs(name).await)
    }
}

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            //TODO: use gumdrop for a fuller command line parser, see cargo-minify
            if let Some(name) = std::env::args().nth(1) {
                display_vital_signs([name.as_str()]).await;
            } else {
                let metadata = MetadataCommand::new().no_deps().exec().unwrap();
                display_vital_signs(
                    metadata.packages[0]
                        .dependencies
                        .iter()
                        .map(|dep| dep.name.as_str()),
                )
                .await;
            };
        })
}
