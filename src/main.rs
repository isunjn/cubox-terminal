use std::env;
use std::process;

use serde::{ Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct UserCfg {
    api_key: Option<String>,
}

impl std::default::Default for UserCfg {
    fn default() -> Self {
        Self { api_key: None }
    }
}

fn main() {
    let mut args = env::args();
    
    let api_key = config_api_key(&mut args);
    
    println!("Your api key is: {}", api_key);
}

fn config_api_key(args: &mut env::Args) -> String {
    match args.nth(1) {
        Some(command) => {
            if command == "setapi" {
                if let Some(api_key) = args.nth(2) {
                    confy::store("cubox", UserCfg { api_key: Some(api_key) }).unwrap();
                    println!("✓ API key set!");
                    process::exit(0);
                } else {
                    println!("Error: Expect api key");
                    process::exit(1);
                }
            }
        },
        None => {
            println!("Error: Expect a subcommand or some text to save to cubox");
            process::exit(1);
        }
    }

    let cfg: UserCfg = confy::load("cubox").unwrap();

    match cfg.api_key {
        Some(api_key) => return api_key,
        None => {
            println!("Error: API key not set!\nPlease use 'cu setapi xxxxxxx' to set your API key");
            process::exit(1);
        }
    }
}