use std::time::Duration;
use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Manifest {
    pub package: Package
}

#[derive(Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub url: String
}

pub async fn download_package(package_name: String) -> Result<()> {
    let manifest = fetch_manifest(package_name).await?;

    Ok(())
}

async fn fetch_manifest(package_name: String) -> Result<Manifest> {
    let spinner = indicatif::ProgressBar::new_spinner().with_message("Fetching manifest...");

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
