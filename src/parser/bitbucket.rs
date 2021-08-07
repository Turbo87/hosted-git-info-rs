use super::{ParsedSegments, Parser};
use crate::{ParseError, Provider};
use std::str;
use url::Url;

#[derive(Debug, Eq, PartialEq)]
pub struct BitbucketParser {}

impl Parser for BitbucketParser {
    fn provider(&self) -> Provider {
        Provider::BitBucket
    }

    fn supports_scheme(&self, scheme: &str) -> bool {
        matches!(scheme, "git+ssh" | "git+https" | "ssh" | "https")
    }

    fn extract<'a>(&self, url: &'a Url) -> Result<ParsedSegments<'a>, ParseError> {
        // let [, user, project, aux] = url.pathname.split('/', 4)
        let mut path_segments = url.path().split('/');
        let _ = path_segments.next();
        let user = path_segments.next();
        let project = path_segments.next();
        let aux = path_segments.next();

        // if (['get'].includes(aux)) {
        //   return
        // }
        if aux == Some("get") {
            return Err(ParseError::UnknownUrl);
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

        // return { user, project, committish: url.hash.slice(1) }
        let committish = url.fragment();

        // return { user, project, committish }
        Ok(ParsedSegments {
            user,
            project,
            committish,
        })
    }
}
