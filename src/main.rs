use anyhow::{Context, Result};
use clap::Parser;
use ketcindyinstaller::argument_parser::Argument;
use ketcindyinstaller::argument_parser::Subcommand;
use ketcindyinstaller::package_manager::download_package;

#[tokio::main]
async fn main() -> Result<()> {
    let project_directories =
        directories::ProjectDirs::from("", "NXVZBGBFBEN", "KeTCindyInstaller").context(
            "No valid home directory path could be retrieved from the operating system.",
        )?;

    let argument = Argument::parse();
    match argument.subcommand {
        Subcommand::Install {
            nodeps: _,
            packages,
        } => {
            for package in packages {
                let download_location = project_directories
                    .cache_dir()
                    .join("packages")
                    .join(package.to_string());
                download_package(package.to_string(), &download_location).await?;
            }
        }
    }

    Ok(())
}
