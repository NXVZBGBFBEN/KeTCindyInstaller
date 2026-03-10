mod fetcher;

use crate::package::PackageKind;

use anyhow::Result;

pub(crate) async fn fetch_versions(package_kind: PackageKind) -> Result<Vec<String>> {
    let versions = match package_kind {
        PackageKind::KeTCindy => {
            let releases = fetcher::fetch_from_github("ketpic", "ketcindy").await?;
            releases.into_iter().map(|release| release.tag_name).collect()
        },
        PackageKind::Cinderella => {
            let release = fetcher::fetch_from_homebrew("cinderella").await?;
            vec![release.version]
        },
        PackageKind::R => {
            let release = fetcher::fetch_from_homebrew("r").await?;
            vec![release.version]
        },
        PackageKind::Maxima => {
            let release = fetcher::fetch_from_homebrew("maxima").await?;
            vec![release.version]
        },
    };

    Ok(versions)
}
