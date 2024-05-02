use std::{fs::File, io::Read};

use bitcoincore_rpc::{Auth, Client};
use serde::Deserialize;
use serde_json::from_str;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rpc_username: String,
    pub rpc_password: String,
    pub rpc_url: String,
}

pub fn get_rpc_client_from_config() -> Client {
    let mut file = File::open("config.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let config: Config = from_str(&contents).unwrap();

    let auth = Auth::UserPass(config.rpc_username.clone(), config.rpc_password.clone());

    Client::new(&format!("{}", config.rpc_url), auth).unwrap()
}

