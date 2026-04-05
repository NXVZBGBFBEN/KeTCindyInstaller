use anyhow::Result;

pub(super) async fn fetch_from_github(
    owner: &str,
    repository: &str,
) -> Result<Vec<octocrab::models::repos::Release>> {
    Ok(octocrab::instance()
        .repos(owner, repository)
        .releases()
        .list()
        .send()
        .await?
        .take_items())
}

#[derive(serde::Deserialize)]
pub(super) struct HomebrewResponse {
    pub(super) version: String,
}

#[derive(serde::Deserialize)]
struct CaskJson {
    version: String,
}

#[derive(serde::Deserialize)]
struct FormulaJson {
    versions: Versions,
}

#[derive(serde::Deserialize)]
struct Versions {
    stable: String,
}

pub(super) async fn fetch_from_homebrew(package_name: &str) -> Result<HomebrewResponse> {
    // 1. try cask
    let cask_url = format!("https://formulae.brew.sh/api/cask/{}.json", package_name);
    let response = reqwest::get(cask_url).await?;
    if response.status() == reqwest::StatusCode::OK {
        let parsed_response = response.json::<CaskJson>().await?;
        return Ok(HomebrewResponse {
            version: parsed_response.version,
        });
    }

    // 2. try formula
    let formula_url = format!("https://formulae.brew.sh/api/formula/{}.json", package_name);
    let response = reqwest::get(formula_url).await?;
    if response.status() == reqwest::StatusCode::OK {
        let parsed_response = response.json::<FormulaJson>().await?;
        return Ok(HomebrewResponse {
            version: parsed_response.versions.stable,
        });
    }

    // not found
    anyhow::bail!("Package not found.");
}
