use anyhow::Result;
use indicatif::ProgressBar;
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize, Debug)]
pub(crate) struct Manifest {
    pub(crate) package: Package,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Package {
    #[allow(dead_code)]
    pub(crate) name: String,
    #[allow(dead_code)]
    pub(crate) version: String,
    pub(crate) target: PackageTarget,
}

#[derive(Deserialize, Debug)]
pub(crate) struct PackageTarget {
    pub(crate) universal: Option<PackageMetadata>,
    pub(crate) amd64_windows: Option<PackageMetadata>,
    pub(crate) arm64_macos: Option<PackageMetadata>,
    pub(crate) amd64_macos: Option<PackageMetadata>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct PackageMetadata {
    pub(crate) url: String,
}

impl Manifest {
    pub(crate) async fn fetch(package_name: &str) -> Result<Self> {
        let spinner = ProgressBar::new_spinner().with_message("Fetching manifest...");

        spinner.enable_steady_tick(Duration::from_millis(75));

        let raw_manifest = reqwest::get(format!("https://raw.githubusercontent.com/NXVZBGBFBEN/KeTCindyInstaller/manifests/manifests/{package_name}.toml"))
            .await?
            .error_for_status()?
            .text()
            .await?;
        let manifest = toml::from_str::<Manifest>(raw_manifest.as_str())?;

        spinner.finish();

        Ok(manifest)
    }
}
