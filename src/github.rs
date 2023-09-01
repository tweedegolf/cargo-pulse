#[derive(Debug)]
pub struct GhStats {
    pub stargazers: u32,
    //pub watchers: u32,
    pub authors: u32,
}

pub async fn fetch_github_data(url: &str) -> Result<GhStats, octocrab::Error> {
    let (owner, repo);
    {
        let mut iter = url.split('/').rev().take(2);
        repo = iter.next().unwrap();
        owner = iter.next().unwrap();
    }

    let gh_data = octocrab::instance().repos(owner, repo).get().await?;

    let gh_contributors: Vec<octocrab::models::Author> = {
        let gh = octocrab::instance();
        let page = gh
            .get(format!("/repos/{owner}/{repo}/contributors"), None::<&()>)
            .await?;

        gh.all_pages(page).await.unwrap()
    };

    Ok(GhStats {
        stargazers: gh_data.stargazers_count.unwrap_or_default(),
        //watchers: gh_data.watchers_count.unwrap_or_default(),  // this data is incorrect in octocrab
        authors: gh_contributors.len() as u32,
    })
}
