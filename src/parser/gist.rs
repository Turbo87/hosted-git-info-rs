use super::{ParsedSegments, Parser};
use crate::{ParseError, Provider};
use std::str;
use url::Url;

#[derive(Debug, Eq, PartialEq)]
pub struct GistParser {}

impl Parser for GistParser {
    fn provider(&self) -> Provider {
        Provider::Gist
    }

    fn supports_scheme(&self, scheme: &str) -> bool {
        matches!(scheme, "git" | "git+ssh" | "git+https" | "ssh" | "https")
    }

    fn extract<'a>(&self, url: &'a Url) -> Result<ParsedSegments<'a>, ParseError> {
        // let [, user, project, aux] = url.pathname.split('/', 4)
        let mut path_segments = url.path().split('/');
        let _ = path_segments.next();
        let mut user = path_segments.next();
        let mut project = path_segments.next();
        let aux = path_segments.next();

        // if (aux === 'raw') {
        //   return
        // }
        if aux == Some("raw") {
            return Err(ParseError::UnknownUrl);
        }

        // if (!project) {
        if project.is_none() || matches!(project, Some(project) if project.is_empty()) {
            // if (!user) {
            if user.is_none() || matches!(user, Some(user) if user.is_empty()) {
                // return
                return Err(ParseError::UnknownUrl);
            }

            project = user;
            user = None;
        }

        // if (project.endsWith('.git')) {
        //   project = project.slice(0, -4)
        // }
        let project = project.map(|project| project.strip_suffix(".git").unwrap_or(project));

        // return { user, project, committish: url.hash.slice(1) }
        let committish = url.fragment();
        Ok(ParsedSegments {
            user,
            project,
            committish,
        })
    }
}
