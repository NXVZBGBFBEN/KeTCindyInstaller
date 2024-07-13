use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;

#[derive(Parser)]
#[command(name = "KeTCindyInstaller", version, about)]
pub struct Arguments {
    #[command(subcommand)]
    subcommand: Subcommands
}

#[derive(Subcommand)]
enum Subcommands {
    /// Install KeTCindy and related packages
    Install {
        #[arg(long)]
        /// Ignore package dependencies
        nodeps: bool,

        #[arg(value_enum, required = true)]
        /// Install a package or a list of packages
        package: Vec<Packages>,
    }
}

#[derive(ValueEnum, Clone)]
enum Packages {
    /// KeTCindy (https://github.com/ketpic/ketcindy)
    Ketcindy,
}

pub fn parse() -> Arguments {
    return Arguments::parse();
}
