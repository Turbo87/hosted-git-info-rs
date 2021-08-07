use super::{ParsedSegments, Parser};
use crate::{ParseError, Provider};
use std::str;
use url::Url;

#[derive(Debug, Eq, PartialEq)]
pub struct GitHubParser {}

impl Parser for GitHubParser {
    fn provider(&self) -> Provider {
        Provider::GitHub
    }

    fn supports_scheme(&self, scheme: &str) -> bool {
        matches!(
            scheme,
            "git" | "http" | "git+ssh" | "git+https" | "ssh" | "https"
        )
    }

    fn extract<'a>(&self, url: &'a Url) -> Result<ParsedSegments<'a>, ParseError> {
        // let [, user, project, type, committish] = url.pathname.split('/', 5)
        let mut path_segments = url.path().splitn(5, '/');
        let _ = path_segments.next();
        let user = path_segments.next();
        let project = path_segments.next();
        let type_ = path_segments.next();
        let mut committish = path_segments.next();

        // if (type && type !== 'tree') {
        //   return
        // }
        //
        // if (!type) {
        //   committish = url.hash.slice(1)
        // }
        if let Some(type_) = type_ {
            if type_ != "tree" {
                return Err(ParseError::UnknownUrl);
            }
        } else {
            committish = url.fragment();
        }

        // if (project && project.endsWith('.git')) {
        //   project = project.slice(0, -4)
        // }
        let project = project.map(|project| project.strip_suffix(".git").unwrap_or(project));

        // if (!user || !project) {
        //   return
        // }
        if user.is_none() || project.is_none() {
            return Err(ParseError::UnknownUrl);
        }

        // return { user, project, committish }
        Ok(ParsedSegments {
            user,
            project,
            committish,
        })
    }
}
