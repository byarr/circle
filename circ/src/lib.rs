use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub token: String
}


pub fn load_config() -> Option<Config> {
    let filename = ".circleci/cli.yml";

    dirs::home_dir()
        .map(|mut home| {home.push(filename); home})
        .filter(|f| f.exists())
        .and_then(|f| File::open(f).ok())
        .map(|f| BufReader::new(f))
        .and_then(|r| serde_yaml::from_reader(r).ok())
}