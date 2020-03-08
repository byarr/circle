use git2::Repository;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub mod git;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub token: String,
}

pub fn load_config() -> Option<Config> {
    let filename = ".circleci/cli.yml";

    dirs::home_dir()
        .map(|mut home| {
            home.push(filename);
            home
        })
        .filter(|f| f.exists())
        .and_then(|f| File::open(f).ok())
        .map(BufReader::new)
        .and_then(|r| serde_yaml::from_reader(r).ok())
}
