use clap::Parser;

#[derive(Parser)]
#[command(name = "KeTCindyInstaller", version, about)]
struct Cli {
}

fn main() {
    let cli = Cli::parse();
}
