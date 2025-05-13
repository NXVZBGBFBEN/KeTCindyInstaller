#[derive(serde::Deserialize)]
pub(crate) struct HomebrewResponse {
    pub(crate) version: String,
}

pub(crate) async fn fetch_github_releases(
    owner: &str,
    repository: &str,
) -> Vec<octocrab::models::repos::Release> {
    octocrab::instance()
        .repos(owner, repository)
        .releases()
        .list()
        .send()
        .await
        .unwrap()
        .take_items()
}

pub(crate) async fn fetch_homebrew_latest_release(name: &str) -> HomebrewResponse {
    reqwest::get("https://formulae.brew.sh/api/cask/cinderella.json")
        .await
        .unwrap()
        .json::<HomebrewResponse>()
        .await
        .unwrap()
}
