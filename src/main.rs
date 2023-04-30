use ketcindyinstaller::config::{Config, SystemConfig};

fn main() {
    println!("This is KeTCindyInstaller, Version 0.1.0-alpha");
    println!("System:");
    let system = match SystemConfig::new() {
        Ok(x) => x,
        Err(e) => {
            println!("ERR!{e}");
            return;
        }
    };
    println!("{:?}", system);
    let config = Config {
        system,
    };
    println!("{:?}", config);
    println!("Path:");
}
