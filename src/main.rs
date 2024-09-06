use ketcindyinstaller::argument_parser;
use ketcindyinstaller::argument_parser::Subcommand;
use ketcindyinstaller::package_manager::download_package;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let argument = argument_parser::parse_argument();
    match argument.subcommand {
        Subcommand::Install { nodeps: _, packages } => {
            for package in packages {
                let manifest = download_package(package.to_string()).await?;
                println!("{:?}", manifest);
            }
        }
    }

    Ok(())
}
