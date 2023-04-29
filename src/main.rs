use ketcindyinstaller::system_config::SystemConfig;

fn main() {
    println!("This is KeTCindyInstaller, Version 0.1.0-alpha");
    println!("System:");
    match SystemConfig::new() {
        Ok(x) => println!("{:?}", x),
        Err(e) => println!("ERR! {e}"),
    }
}
