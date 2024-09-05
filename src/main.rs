use ketcindyinstaller::argument_parser;
use ketcindyinstaller::argument_parser::Subcommand;
use ketcindyinstaller::manifest_parser::parse_manifest;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let argument = argument_parser::parse_argument();
    match argument.subcommand {
        Subcommand::Install { nodeps: _, packages } => {
            for package in packages {
                let manifest = parse_manifest(package.to_string()).await?;
                println!("{:?}", manifest.package.url);
            }
        }
    }

    Ok(())
}
