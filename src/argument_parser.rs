#[derive(clap::Parser)]
#[command(name = "KeTCindyInstaller", version, about)]
pub struct Argument {
    #[command(subcommand)]
    pub subcommand: Subcommand,
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
    },
}

#[derive(clap::ValueEnum, strum::Display, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum Package {
    /// KeTCindy (https://github.com/ketpic/ketcindy)
    Ketcindy,
    /// Cinderella (https://cinderella.de/)
    Cinderella,
    /// Maxima (https://maxima.sourceforge.io/)
    Maxima,
    /// R (https://cran.r-project.org/)
    R,
}
