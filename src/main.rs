use cargo_metadata::MetadataCommand;
use colored::Colorize;

mod crates_io;
use crates_io::GetVitalSigns;
mod github;

async fn display_vital_signs<'a>(crates: impl IntoIterator<Item = &'a str>) {
    let mut client = crates_io::spawn();
    println!(
        "{:<24} {:<12} {:<14} {:<6} {:<10} {:<10} {:<7} {:<6}",
        "crate", "age", "updated", "pulse", "downloads", "rev. deps", "authors", "stars"
    );
    for name in crates {
        //TODO: use color in selected areas to give subjective indications
        let health = client.get_vital_signs(name).await.unwrap();
        let name = name.to_string().green();
        let age = format!("{} days", health.full_age.num_days()).yellow();
        let stale = format!("{} days ago", health.staleness.num_days()).bright_yellow();
        let pulse = format!("{:.1}", health.update_frequency).bright_green();
        let dlds = format!("{:.1}", health.recent_downloads).bright_cyan();
        let deps = health
            .reverse_dependencies
            .map(|deps| format!("{:.1}", deps).bright_magenta())
            .unwrap_or("n/a".magenta());
        let gh_author = health
            .authors
            .map(|deps| format!("{:.1}", deps).bright_white())
            .unwrap_or("n/a".white());
        let gh_stargaze = health
            .stargazers
            .map(|deps| format!("{:.1}", deps).bright_blue())
            .unwrap_or("n/a".blue());
        println!(
            "{:<24} {:<12} {:<14} {:<6} {:<10} {:<10} {:<7} {:<6}",
            name, age, stale, pulse, dlds, deps, gh_author, gh_stargaze
        );
    }
}

#[tokio::main]
async fn main() {
    // Drop the first actual argument if it is equal to our subcommand
    // (i.e. we are being called via 'cargo')
    let mut args = std::env::args().peekable();
    args.next();

    if args.peek().map(|s| s.as_str()) == Some("pulse") {
        args.next();
    }

    if let Some(name) = args.next() {
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
}
