use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {    
    pub port: u16,
    pub address: String,
    pub database: Database,
    pub logging: Logging,  
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Database{
    pub connection: Connection,    
    pub scripts: Scripts,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Connection {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: i32,
    pub database_name: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Scripts{
    pub build: String,
    pub drop: String,
    pub update: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Logging {
    pub level: String,
    pub log_to_file: bool,
    pub file_path: String,
    pub append_file: bool,
    pub overwrite_file: bool,
}