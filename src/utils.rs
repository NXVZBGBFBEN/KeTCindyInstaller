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
