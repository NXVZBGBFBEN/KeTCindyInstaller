use clap::Parser;

#[derive(clap::Parser)]
#[command(name = "KeTCindyInstaller", version, about)]
pub struct Arguments {
    #[command(subcommand)]
    pub subcommand: Subcommand
}

#[derive(clap::Subcommand)]
pub enum Subcommand {
    /// Install KeTCindy and related packages
    Install {
        #[arg(long)]
        /// Ignore package dependencies
        nodeps: bool,

        #[arg(value_enum, required = true)]
        /// Install a package or a list of packages
        packages: Vec<Package>,
    }
}

#[derive(clap::ValueEnum, Clone)]
pub enum Package {
    /// KeTCindy (https://github.com/ketpic/ketcindy)
    Ketcindy,
}

pub fn parse() -> Arguments {
    return Arguments::parse();
}
