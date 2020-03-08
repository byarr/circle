use git2::Repository;
use std::path::Path;


pub fn origin_url<P: AsRef<Path>>(p: P) -> Option<String> {

    Repository::discover(p).ok().and_then(|r| {
        r.find_remote("origin")
            .ok()
            .and_then(|remote| remote.url().map(|s| s.to_string()))
    })
}


pub fn to_slug(url: &str) -> Option<String> {
    let e = regex::Regex::new(r"[:|/]([a-zA-Z0-9_-]+)/([a-zA-Z0-9_-]+).git$").unwrap();
    e.captures(url).map(|c| format!("gh/{}/{}", &c[1], &c[2]))
}

#[derive(Debug)]
pub struct RepoInfo {
    pub branch: Option<String>,
    pub origin_url: Option<String>,
}

impl RepoInfo {
    pub fn from_path<P: AsRef<Path>>(p: P) -> Option<Self> {
        let repo = Repository::discover(p).ok()?;

        let head = repo.head().ok();
        let branch = head.and_then(|h| h.shorthand().map(|s| s.to_string()));

        let origin_url = repo.find_remote("origin").ok().and_then(|o| o.url().map(|s| s.to_string()));

        Some(RepoInfo {
            origin_url,
            branch,
        })
    }

    pub fn slug(&self) -> Option<String> {
        self.origin_url.as_ref().and_then(|s| to_slug(&s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_slug_works() {
        assert_eq!(
            to_slug("https://github.com/USERNAME/REPOSITORY.git"),
            Some("gh/USERNAME/REPOSITORY".to_string())
        );
        assert_eq!(
            to_slug("git@github.com:USERNAME/REPOSITORY.git"),
            Some("gh/USERNAME/REPOSITORY".to_string())
        );
        assert_eq!(
            to_slug("https://github.com/name-with_some-chars/repo_with-some--chars.git"),
            Some("gh/name-with_some-chars/repo_with-some--chars".to_string())
        );
    }
}
