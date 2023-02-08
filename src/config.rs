use std::env;
use ethcontract::PrivateKey;

#[derive(Clone, Debug)]
pub struct Config {
    pub listen_url: String,  
    pub private_key: PrivateKey,
    pub chain_id: u64,
    pub moralis_base_url: String,
    pub moralis_api_key: String,
}

pub fn init() -> Config { 
    let panic_message: String = "enviroment variable is not set".to_string();

    Config {
        listen_url: match env::var("LISTEN_URL") {
            Ok(var) => var,
            Err(_) => panic!("LISTEN_URL {}", panic_message)
        },
        private_key: match env::var("PRIVATE_KEY") {
            Ok(var) => var.parse().expect("invalid PK"),
            Err(_) => panic!("PRIVATE_KEY {}", panic_message)
        },
        chain_id: match env::var("CHAIN_ID") {
            Ok(var) => var.parse::<u64>().unwrap(),
            Err(_) => panic!("CHAIN_ID {}", panic_message)
        },
        moralis_base_url: match env::var("MORALIS_BASE_URL") {
            Ok(var) => var,
            Err(_) => panic!("MORALIS_BASE_URL {}", panic_message)
        },
        moralis_api_key: match env::var("MORALIS_API_KEY") {
            Ok(var) => var,
            Err(_) => panic!("MORALIS_API_KEY {}", panic_message)
        },
    }
}
