use ketcindyinstaller::config::{Config, SystemConfig};

use console::Term;
use dialoguer::Confirm;

fn main() {
    let stdout = Term::stdout();
    println!("This is KeTCindyInstaller, Version 0.1.0-alpha");
    println!("System:");
    let mut system: SystemConfig;
    loop {
        system = match SystemConfig::new() {
            Ok(x) => x,
            Err(e) => {
                println!("ERR! {e}");
                return;
            }
        };
        stdout.write_line(&format!("{}", system)).unwrap();
        if Confirm::new()
            .with_prompt("proceed?")
            .report(false)
            .default(true)
            .interact()
            .unwrap()
        {
            break;
        } else {
            stdout.clear_last_lines(3).unwrap();
            continue;
        }
    }
    /*
    let config = Config {
        system,
    };
    println!("{}", config);
     */
    //println!("Path:");
}
