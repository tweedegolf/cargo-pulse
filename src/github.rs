#[derive(Debug)]
pub struct GhStats {
    pub stargazers: u32,
    //pub watchers: u32,
    pub authors: u32,
}

pub fn fetch_github_data(url: &str) -> Option<GhStats> {
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

    let gh_contributors: Vec<octocrab::models::Author> = runtime.block_on(async {
        let gh = octocrab::instance();
        let page = gh
            .get(format!("/repos/{owner}/{repo}/contributors"), None::<&()>)
            .await
            .unwrap();

        gh.all_pages(page).await.unwrap()
    });

    Some(GhStats {
        stargazers: gh_data.stargazers_count.unwrap_or_default(),
        //watchers: gh_data.watchers_count.unwrap_or_default(),  // this data is incorrect in octocrab
        authors: gh_contributors.len() as u32,
    })
}
