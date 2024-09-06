use std::{io::Write, time::Duration};
use std::path::Path;
use std::fs;
use anyhow::{Context, Result};
use serde::Deserialize;
use indicatif::ProgressBar;

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

pub async fn download_package(package_name: String, download_location: &Path) -> Result<()> {
    let manifest = fetch_manifest(package_name).await?;
    let package_file_location = download_location.join(url::Url::parse(&manifest.package.url)?
        .path_segments().context("Failed to parse package URL.")?
        .last().context("Failed to parse package URL.")?);
    let spinner = ProgressBar::new_spinner().with_message("Downloading package...");

    spinner.enable_steady_tick(Duration::from_millis(75));

    let package = reqwest::get(&manifest.package.url)
        .await?
        .error_for_status()?
        .bytes()
        .await?;
    fs::create_dir_all(download_location)?;
    fs::File::create(package_file_location)?.write_all(&package)?;

    spinner.finish();

    Ok(())
}

async fn fetch_manifest(package_name: String) -> Result<Manifest> {
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
