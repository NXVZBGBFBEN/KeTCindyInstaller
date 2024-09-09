use crate::manifest_manager::Manifest;
use anyhow::{Context, Result};
use indicatif::ProgressBar;
use std::fs;
use std::path::Path;
use std::{io::Write, time::Duration};

pub async fn download_package(package_name: &str, download_location: &Path) -> Result<()> {
    if cfg!(all(target_arch = "aarch64", target_os = "macos")) && package_name == "maxima" {
        eprintln!(
            "installation of package \"maxima\" is not supported in this cpu architecture (arm64)."
        );
        eprintln!("Please install with Homebrew: `brew install maxima`.");
        return Ok(());
    }

    let manifest = Manifest::fetch(package_name).await?;
    let package_metadata = if let Some(universal_metadata) = manifest.package.target.universal {
        universal_metadata
    } else if cfg!(all(target_arch = "x86_64", target_os = "windows")) {
        manifest
            .package
            .target
            .amd64_windows
            .context("Manifest format is invalid.")?
    } else if cfg!(all(target_arch = "aarch64", target_os = "macos")) {
        manifest
            .package
            .target
            .arm64_macos
            .context("Manifest format is invalid.")?
    } else if cfg!(all(target_arch = "x86_64", target_os = "macos")) {
        manifest
            .package
            .target
            .amd64_macos
            .context("Manifest format is invalid.")?
    } else {
        unreachable!("This feature is not available in this build");
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
