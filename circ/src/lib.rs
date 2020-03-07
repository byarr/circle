use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::BufReader;
use git2::Repository;
use std::path::Path;

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

pub fn origin_url<P: AsRef<Path>>(p: P) -> Option<String> {

    Repository::discover(p).ok()
        .and_then(|r| url_for_repo(r))

}

fn url_for_repo(r: Repository) -> Option<String> {
    r.find_remote("origin").ok().and_then(|remote| remote.url().map(|s| s.to_string()))
}

fn to_slug(url: &str) -> Option<String> {

    let e = regex::Regex::new(r"[:|/]([a-zA-Z0-9_-]+)/([a-zA-Z0-9_-]+).git$").unwrap();
    e.captures(url)
        .map(|c| format!("gh/{}/{}", &c[1], &c[2]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_slug_works() {
        assert_eq!( to_slug("https://github.com/USERNAME/REPOSITORY.git"),
                    Some("gh/USERNAME/REPOSITORY".to_string()));
        assert_eq!( to_slug("git@github.com:USERNAME/REPOSITORY.git"),
                    Some("gh/USERNAME/REPOSITORY".to_string()));
        assert_eq!( to_slug("https://github.com/name-with_some-chars/repo_with-some--chars.git"),
                    Some("gh/name-with_some-chars/repo_with-some--chars".to_string()));
    }
}

