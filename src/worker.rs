mod downloader;
mod fetcher;
mod installer;

use crate::package::PackageKind;

use anyhow::Result;
use anyhow::Context;
use std::path::PathBuf;

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

pub(crate) async fn download_package<F>(
    package_kind: PackageKind,
    version: &str,
    progress_callback: F
) -> Result<PathBuf>
where
    F: FnMut(f32) + Send,
{
    use url::Url;

    let mut download_destination = directories::ProjectDirs::from("", "nxvzbgbfben", "ketcindyinstaller")
        .context("Failed to create the project directory")?.data_dir().join("downloads");
    std::fs::create_dir_all(&download_destination)?;

    let download_url = match package_kind {
        PackageKind::KeTCindy => {
            download_destination.push(format!("{package_kind}_{version}.zip"));
            Url::parse(&format!("https://github.com/ketpic/ketcindy/archive/refs/tags/{version}.zip"))?
        },
        PackageKind::Cinderella => {
            if cfg!(target_os = "windows") {
                download_destination.push(format!("{package_kind}_{version}.exe"));
                Url::parse(&format!("https://beta.cinderella.de/Cinderella-{version}-64bit.exe"))?
            } else if cfg!(target_os = "macos") {
                download_destination.push(format!("{package_kind}_{version}.dmg"));
                Url::parse(&format!("https://beta.cinderella.de/Cinderella-{version}.dmg"))?
            } else {
                unimplemented!()
            }
        },
        PackageKind::R => {
            if cfg!(target_os = "windows") {
                download_destination.push(format!("{package_kind}_{version}.exe"));
                Url::parse(&format!("https://cran.r-project.org/bin/windows/base/R-{version}-win.exe"))?
            } else if cfg!(target_os = "macos") && cfg!(target_arch = "aarch64") {
                download_destination.push(format!("{package_kind}_{version}.pkg"));
                Url::parse(&format!("https://cran.r-project.org/bin/macosx/big-sur-arm64/base/R-{version}-arm64.pkg"))?
            } else if cfg!(target_os = "macos") && cfg!(target_arch = "x86_64") {
                download_destination.push(format!("{package_kind}_{version}.pkg"));
                Url::parse(&format!("https://cran.r-project.org/bin/macosx/big-sur-x86_64/base/R-{version}-x86_64.pkg"))?
            } else {
                unimplemented!()
            }
        },
        PackageKind::Maxima => {
            if cfg!(target_os = "windows") {
                download_destination.push(format!("{package_kind}_{version}.exe"));
                Url::parse(&format!("https://sourceforge.net/projects/maxima/files/Maxima-Windows/{version}-Windows/maxima-{version}-win64.exe/download"))?
            } else if cfg!(target_os = "macos") {
                download_destination.push(format!("{package_kind}_5.46.0.dmg"));
                Url::parse(&format!("https://sourceforge.net/projects/maxima/files/Maxima-MacOS/5.46.0-macOS/MacPorts-Maxima-5.46.0.dmg/download"))?
            } else {
                unimplemented!()
            }
        },
    };

    let package_path = downloader::download_file(download_url, download_destination, progress_callback).await?;

    Ok(package_path)
}

pub(crate) async fn install_package(package_kind: PackageKind, source_path: PathBuf) -> Result<()> {
    match package_kind {
        PackageKind::KeTCindy => {
            let install_directory = if cfg!(target_os = "windows") {
                PathBuf::from("C:\\ketcindy")
            } else if cfg!(target_os = "macos") {
                directories::ProjectDirs::from("", "nxvzbgbfben", "ketcindyinstaller")
                    .context("Failed to create the project directory")?.data_dir().join("packages").join("ketcindy")
            } else {
                unimplemented!();
            };

            std::fs::create_dir_all(&install_directory)?;
            installer::install_zip(source_path, install_directory).await?;
            tokio::process::Command::new("explorer").args(["C:\\ketcindy\\doc"]).status().await?;
        },
        PackageKind::Cinderella => {
            #[cfg(target_os = "windows")]
            installer::install_exe(source_path).await?;
            #[cfg(target_os = "macos")]
            installer::install_dmg(source_path).await?;
        },
        PackageKind::R => {
            #[cfg(target_os = "windows")]
            installer::install_exe(source_path).await?;
            #[cfg(target_os = "macos")]
            installer::install_pkg(source_path).await?;
        },
        PackageKind::Maxima => {
            #[cfg(target_os = "windows")]
            installer::install_exe(source_path).await?;
            #[cfg(target_os = "macos")]
            installer::install_dmg(source_path).await?;
        },
    }
    Ok(())
}
