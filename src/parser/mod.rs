use self::bitbucket::BitbucketParser;
use self::gist::GistParser;
use self::github::GitHubParser;
use self::gitlab::GitLabParser;
use crate::{ParseError, Provider};
use std::fmt::{Debug, Formatter};
use std::str;
use url::Url;

mod bitbucket;
mod gist;
mod github;
mod gitlab;

pub fn parser_from_shortcut(shortcut: &str) -> Option<Box<dyn Parser>> {
    match shortcut {
        "bitbucket" => Some(Box::new(BitbucketParser {})),
        "gist" => Some(Box::new(GistParser {})),
        "github" => Some(Box::new(GitHubParser {})),
        "gitlab" => Some(Box::new(GitLabParser {})),
        _ => None,
    }
}

pub fn parser_from_domain(domain: &str) -> Option<Box<dyn Parser>> {
    match domain {
        "bitbucket.org" => Some(Box::new(BitbucketParser {})),
        "gist.github.com" => Some(Box::new(GistParser {})),
        "github.com" => Some(Box::new(GitHubParser {})),
        "gitlab.com" => Some(Box::new(GitLabParser {})),
        _ => None,
    }
}

pub trait Parser {
    fn provider(&self) -> Provider;
    fn supports_scheme(&self, scheme: &str) -> bool;
    fn extract<'a>(&self, url: &'a Url) -> Result<ParsedSegments<'a>, ParseError>;
}

impl Debug for dyn Parser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.provider())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ParsedSegments<'a> {
    pub user: Option<&'a str>,
    pub project: Option<&'a str>,
    pub committish: Option<&'a str>,
}
