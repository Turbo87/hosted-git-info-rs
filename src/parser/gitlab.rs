use super::{ParsedSegments, Parser};
use crate::{ParseError, Provider};
use std::str;
use url::Url;

#[derive(Debug, Eq, PartialEq)]
pub struct GitLabParser {}

impl Parser for GitLabParser {
    fn provider(&self) -> Provider {
        Provider::GitLab
    }

    fn supports_scheme(&self, scheme: &str) -> bool {
        matches!(scheme, "git+ssh" | "git+https" | "ssh" | "https")
    }

    fn extract<'a>(&self, url: &'a Url) -> Result<ParsedSegments<'a>, ParseError> {
        // const path = url.pathname.slice(1)
        let path = &url.path()[1..];

        // if (path.includes('/-/') || path.includes('/archive.tar.gz')) {
        //   return
        // }
        if path.contains("/-/") || path.contains("/archive.tar.gz") {
            return Err(ParseError::UnknownUrl);
        }

        // const segments = path.split('/')
        let mut segments = path.rsplitn(2, '/');
        // let project = segments.pop()
        let project = segments.next();
        // if (project.endsWith('.git')) {
        //   project = project.slice(0, -4)
        // }
        let project = project.map(|project| project.strip_suffix(".git").unwrap_or(project));
        // const user = segments.join('/')
        let user = segments.next();

        // if (!user || !project) {
        //   return
        // }
        if user.is_none() || project.is_none() {
            return Err(ParseError::UnknownUrl);
        }

        // return { user, project, committish: url.hash.slice(1) }
        let committish = url.fragment();
        Ok(ParsedSegments {
            user,
            project,
            committish,
        })
    }
}
