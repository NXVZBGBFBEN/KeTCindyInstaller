use anyhow::{Context, Result};
use indicatif::ProgressBar;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::{io::Write, time::Duration};

#[derive(Deserialize, Debug)]
struct Manifest {
    package: Package,
}

#[derive(Deserialize, Debug)]
struct Package {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    version: String,
    target: PackageTarget,
}

#[derive(Deserialize, Debug)]
struct PackageTarget {
    universal: Option<PackageMetadata>,
}

#[derive(Deserialize, Debug)]
struct PackageMetadata {
    url: String,
}

pub async fn download_package(package_name: String, download_location: &Path) -> Result<()> {
    let manifest = fetch_manifest(package_name).await?;
    let package_metadata = if let Some(metadata) = manifest.package.target.universal {
        metadata
    } else {
        todo!("parse metadata (OS, arch)")
    };
    let package_file_location = download_location.join(
        url::Url::parse(&package_metadata.url)?
            .path_segments()
            .context("Failed to parse package URL.")?
            .last()
            .context("Failed to parse package URL.")?,
    );
    let spinner = ProgressBar::new_spinner().with_message("Downloading package...");

    spinner.enable_steady_tick(Duration::from_millis(75));

    let package = reqwest::get(&package_metadata.url)
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
