use std::{fs::File, io::Read};

use bitcoincore_rpc::{Auth, Client};
use serde::Deserialize;
use serde_json::from_str;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rpc_cookie_path: Option<String>,
    pub rpc_username: Option<String>,
    pub rpc_password: Option<String>,
    pub rpc_url: String,
}

pub fn get_rpc_client_from_config() -> Client {
    let mut file = File::open("config.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let config: Config = from_str(&contents).unwrap();

    let auth = match (
        config.rpc_username,
        config.rpc_password,
        config.rpc_cookie_path,
    ) {
        (Some(username), Some(password), None) => Auth::UserPass(username, password),
        (None, None, Some(cookie_path)) => Auth::CookieFile(cookie_path.into()),
        _ => panic!("neither cookie path nor credentials specified"),
    };

    Client::new(&format!("{}", config.rpc_url), auth).unwrap()
}
