mod fetcher;

use std::sync::Mutex;
use anyhow::Result;

pub(crate) enum PackageKind {
    KeTCindy,
    Cinderella,
    R,
    Maxima,
}

pub(crate) struct Package {
    kind: PackageKind,
    pub(crate) versions: Mutex<Option<Vec<String>>>,
    pub(crate) selected_version_index: Mutex<usize>,
}

impl Package {
    pub(crate) fn new(kind: PackageKind) -> Self {
        Self {
            kind: kind,
            versions: Mutex::new(None),
            selected_version_index: Mutex::new(0),
        }
    }

    pub(crate) async fn fetch_versions(&self) -> Result<()> {
        let versions = match self.kind {
            PackageKind::KeTCindy => {
                let releases = fetcher::fetch_from_github("ketpic", "ketcindy").await?;
                Some(releases.into_iter().map(|release| release.tag_name).collect())
            },
            PackageKind::Cinderella => {
                let release = fetcher::fetch_from_homebrew("cinderella").await?;
                Some(vec![release.version])
            }
            PackageKind::R => {
                let release = fetcher::fetch_from_homebrew("r").await?;
                Some(vec![release.version])
            },
            PackageKind::Maxima => {
                let release = fetcher::fetch_from_homebrew("maxima").await?;
                Some(vec![release.version])
            }
        };
        *self.versions.lock().unwrap() = versions;

        Ok(())
    }

    // pub(crate) fn selected_version(&self) -> Option<&String> {
    //     self.versions.as_ref().and_then(|versions| versions.get(self.selected_version_index))
    // }
}
