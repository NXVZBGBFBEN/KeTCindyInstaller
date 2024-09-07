use anyhow::{Context, Result};
use indicatif::ProgressBar;
use std::fs;
use std::path::Path;
use std::{io::Write, time::Duration};

pub async fn download_package(package_name: String, download_location: &Path) -> Result<()> {
    let manifest = crate::manifest_manager::fetch_manifest(package_name).await?;
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
