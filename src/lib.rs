//! hosted-git-info is a [Rust] port of the original [hosted-git-info] project on [npm].
//!
//! [Rust]: https://www.rustlang.org/
//! [hosted-git-info]: https://github.com/npm/hosted-git-info
//! [npm]: https://www.npmjs.com
//!
//! It provides metadata and conversions from repository urls for [GitHub], [Bitbucket]
//! and [GitLab].
//!
//! [GitHub]: https://github.com/
//! [Bitbucket]: https://www.bitbucket.org/
//! [GitLab]: https://www.gitlab.com/
//!
//! It will let you identify and transform various git hosts URLs between
//! protocols. It also can tell you what the URL is for the raw path for
//! particular file for direct access without git.
//!
//! # Usage
//!
//! First, URL parsing may fail for various reasons and therefore returns a `Result`:
//!
//! ```
//! use hosted_git_info::{HostedGitInfo, ParseError};
//!
//! assert!(HostedGitInfo::from_url("https://www.rustlang.org/") == Err(ParseError::UnknownUrl));
//! ```
//!
//! Let’s parse a valid URL and look at its components.
//!
//! ```
//! use hosted_git_info::{HostedGitInfo, Provider};
//!
//! let url = "https://github.com/foo/bar.git#branch";
//! let info = HostedGitInfo::from_url(url).unwrap();
//! assert_eq!(info.provider(), Provider::GitHub);
//! assert_eq!(info.user(), Some("foo"));
//! assert_eq!(info.project(), "bar");
//! assert_eq!(info.committish(), Some("branch"));
//! assert_eq!(info.auth(), None);
//! ```
//!
//! [HostedGitInfo] also implements the [str::FromStr] trait:
//!
//! ```
//! use hosted_git_info::{HostedGitInfo, Provider};
//!
//! let url = "git+ssh://github.com:foo/bar.git";
//! let info: HostedGitInfo = url.parse().unwrap();
//! assert_eq!(info.provider(), Provider::GitHub);
//! assert_eq!(info.user(), Some("foo"));
//! assert_eq!(info.project(), "bar");
//! assert_eq!(info.committish(), None);
//! assert_eq!(info.auth(), None);
//! ```

#![deny(clippy::unwrap_used)]

#[cfg(feature = "derive_builder")]
use derive_builder::Builder;
use percent_encoding::percent_decode_str;
use std::str;
use thiserror::Error;
use url::Url;

mod parser;

static AUTH_SCHEMES: [&str; 5] = ["git", "https", "git+https", "http", "git+http"];
static KNOWN_SCHEMES: [&str; 10] = [
    "http",
    "https",
    "git",
    "git+ssh",
    "git+https",
    "ssh",
    "bitbucket",
    "gist",
    "github",
    "gitlab",
];

/// Enum of supported git hosting providers.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Provider {
    /// see <https://www.bitbucket.org/>
    BitBucket,
    /// see <https://gist.github.com/>
    Gist,
    /// see <https://github.com/>
    GitHub,
    /// see <https://www.gitlab.com/>
    GitLab,
}

/// Enum of the original URL types (shortcut, https, ssh, ...)
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum DefaultRepresentation {
    /// Example: `Turbo87/hosted-git-info-rs`
    Shortcut,
    /// Example: `git://github.com/Turbo87/hosted-git-info-rs`
    Git,
    /// Example: `https://github.com/Turbo87/hosted-git-info-rs.git`
    Https,
    /// Example: `git+ssh://git@github.com:Turbo87/hosted-git-info-rs.git`
    Ssh,
    /// anything else 🤷‍
    Other,
}

impl DefaultRepresentation {
    fn from_scheme(scheme: &str) -> DefaultRepresentation {
        use DefaultRepresentation::*;

        match scheme {
            "git" => Git,
            "git+https" => Https,
            "git+ssh" => Ssh,
            "https" => Https,
            "ssh" => Ssh,
            _ => Other,
        }
    }
}

/// Errors that can occur during parsing.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Error)]
pub enum ParseError {
    /// Failed to parse the URL with the `url` crate.
    #[error("Failed to parse URL")]
    InvalidUrl(#[from] url::ParseError),

    /// Failed to parse a part of the URL with the `percent_encoding` crate.
    #[error("Failed to parse percent-encoded URI component")]
    InvalidUriEncoding(#[from] str::Utf8Error),

    /// The URL could not be recognized.
    #[error("Failed to recognize URL")]
    UnknownUrl,
}

/// The parsed information from a git hosting URL.
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
pub struct HostedGitInfo {
    provider: Provider,

    #[cfg_attr(
        feature = "derive_builder",
        builder(setter(into, strip_option), default)
    )]
    user: Option<String>,

    #[cfg_attr(
        feature = "derive_builder",
        builder(setter(into, strip_option), default)
    )]
    auth: Option<String>,

    #[cfg_attr(feature = "derive_builder", builder(setter(into)))]
    project: String,

    #[cfg_attr(
        feature = "derive_builder",
        builder(setter(into, strip_option), default)
    )]
    committish: Option<String>,

    #[cfg_attr(feature = "derive_builder", builder(setter(name = "repr")))]
    default_representation: DefaultRepresentation,
}

impl HostedGitInfo {
    /// Parses a URL string and returns a [HostedGitInfo] struct, if successful.
    /// If parsing fails, a [ParseError] will be returned.
    pub fn from_url(giturl: &str) -> Result<Self, ParseError> {
        // if (!giturl) {
        //   return
        // }

        // const url = isGitHubShorthand(giturl) ? 'github:' + giturl : correctProtocol(giturl)
        let url = if is_github_shorthand(giturl) {
            format!("github:{}", giturl)
        } else {
            // correctProtocol(giturl)
            correct_protocol(giturl)
        };

        // const parsed = parseGitUrl(url)
        // if (!parsed) {
        //   return parsed
        // }
        let parsed = parse_git_url(&url)?;

        // const gitHostShortcut = gitHosts.byShortcut[parsed.protocol]
        let parser_from_shortcut = parser::parser_from_shortcut(parsed.scheme());

        // const gitHostDomain = gitHosts.byDomain[parsed.hostname.startsWith('www.') ? parsed.hostname.slice(4) : parsed.hostname]
        let simplified_domain = parsed
            .domain()
            .map(|domain| domain.strip_prefix("www.").unwrap_or(domain));
        let parser_from_domain =
            simplified_domain.and_then(|domain| parser::parser_from_domain(domain));

        // const gitHostName = gitHostShortcut || gitHostDomain
        let parser = parser_from_shortcut
            .as_ref()
            .or_else(|| parser_from_domain.as_ref());

        // if (!gitHostName) {
        //   return
        // }
        //
        // const gitHostInfo = gitHosts[gitHostShortcut || gitHostDomain]
        let parser = match parser {
            Some(parser) => parser,
            None => return Err(ParseError::UnknownUrl),
        };

        // let auth = null
        // if (authProtocols[parsed.protocol] && (parsed.username || parsed.password)) {
        //   auth = `${parsed.username}${parsed.password ? ':' + parsed.password : ''}`
        // }
        let username = match parsed.username() {
            username if !username.is_empty() => Some(username),
            _ => None,
        };
        let password = parsed.password();
        let auth = if AUTH_SCHEMES.contains(&parsed.scheme()) {
            match (username, password) {
                (Some(username), Some(password)) => Some(format!("{}:{}", username, password)),
                (Some(username), None) => Some(username.to_string()),
                (None, Some(password)) => Some(format!(":{}", password)),
                (None, None) => None,
            }
        } else {
            None
        };

        // let committish = null
        // let user = null
        // let project = null
        // let defaultRepresentation = null
        //
        // try {
        //   if (gitHostShortcut) {
        if parser_from_shortcut.is_some() {
            // let pathname = parsed.pathname.startsWith('/') ? parsed.pathname.slice(1) : parsed.pathname
            let path = parsed.path();
            let mut pathname = path.strip_prefix('/').unwrap_or(path);

            // const firstAt = pathname.indexOf('@')
            let first_at = pathname.find('@');
            // we ignore auth for shortcuts, so just trim it out
            // if (firstAt > -1) {
            //   pathname = pathname.slice(firstAt + 1)
            // }
            if let Some(first_at) = first_at {
                pathname = &pathname[first_at + 1..];
            }

            // const lastSlash = pathname.lastIndexOf('/')
            let last_slash = pathname.rfind('/');
            let (user, project) = if let Some(last_slash) = last_slash {
                // user = decodeURIComponent(pathname.slice(0, lastSlash))
                let user = percent_decode_str(&pathname[0..last_slash]).decode_utf8()?;

                // we want nulls only, never empty strings
                // if (!user) {
                //   user = null
                // }
                let user = if user.is_empty() { None } else { Some(user) };

                // project = decodeURIComponent(pathname.slice(lastSlash + 1))
                let project = percent_decode_str(&pathname[last_slash + 1..]).decode_utf8()?;
                (user, project)
            } else {
                // project = decodeURIComponent(pathname)
                let project = percent_decode_str(&pathname).decode_utf8()?;
                (None, project)
            };

            let project = project
                .strip_suffix(".git")
                .unwrap_or_else(|| project.as_ref());

            // if (parsed.hash) {
            //   committish = decodeURIComponent(parsed.hash.slice(1))
            // }
            let committish = parsed
                .fragment()
                .map(|committish| percent_decode_str(&committish).decode_utf8())
                .transpose()?;

            // defaultRepresentation = 'shortcut'
            Ok(Self {
                provider: parser.provider(),
                user: user.map(|s| s.to_string()),
                auth,
                project: project.to_string(),
                committish: committish.map(|s| s.to_string()),
                default_representation: DefaultRepresentation::Shortcut,
            })
        } else {
            // if (!gitHostInfo.protocols.includes(parsed.protocol)) {
            //   return
            // }
            if !parser.supports_scheme(parsed.scheme()) {
                return Err(ParseError::UnknownUrl);
            }

            // const segments = gitHostInfo.extract(parsed)
            // if (!segments) {
            //   return
            // }
            let segments = parser.extract(&parsed)?;

            // user = segments.user && decodeURIComponent(segments.user)
            let user = segments
                .user
                .map(|user| percent_decode_str(&user).decode_utf8())
                .transpose()?;

            // project = decodeURIComponent(segments.project)
            let project = segments
                .project
                .map(|project| percent_decode_str(&project).decode_utf8())
                .transpose()?
                .ok_or(ParseError::UnknownUrl)?;

            // committish = decodeURIComponent(segments.committish)
            let committish = segments
                .committish
                .map(|committish| percent_decode_str(&committish).decode_utf8())
                .transpose()?;

            // defaultRepresentation = protocolToRepresentation(parsed.protocol)
            Ok(Self {
                provider: parser.provider(),
                user: user.map(|s| s.to_string()),
                auth,
                project: project.to_string(),
                committish: committish.map(|s| s.to_string()),
                default_representation: DefaultRepresentation::from_scheme(parsed.scheme()),
            })
        }
        //   }
        // } catch (err) {
        //   /* istanbul ignore else */
        //   if (err instanceof URIError) {
        //     return
        //   } else {
        //     throw err
        //   }
        // }
        //
        // return new GitHost(gitHostName, user, auth, project, committish, defaultRepresentation, opts)
    }

    /// The type of hosting provider. (GitHub, Gitlab, Bitbucket, ...)
    pub fn provider(&self) -> Provider {
        self.provider
    }

    /// The name of the user or organization on the git host.
    ///
    /// This is using an [Option] because some hosting providers allow projects
    /// that are not scoped to a particular user or organization.
    ///
    /// Example: `https://github.com/Turbo87/hosted-git-info-rs.git` → `Turbo87`
    pub fn user(&self) -> Option<&str> {
        self.user.as_deref()
    }

    /// The authentication part of the URL, if it exists.
    ///
    /// Format: `<USER>[:<PASSWORD>]`
    ///
    /// Example: `https://user:password@github.com/foo/bar.git` → `user:password`
    pub fn auth(&self) -> Option<&str> {
        self.auth.as_deref()
    }

    /// The name of the project on the git host.
    ///
    /// Example: `https://github.com/Turbo87/hosted-git-info-rs.git` → `hosted-git-info-rs`
    pub fn project(&self) -> &str {
        &self.project
    }

    /// The [branch, tag, commit, ...](https://git-scm.com/docs/gitglossary#Documentation/gitglossary.txt-aiddefcommit-ishacommit-ishalsocommittish)
    /// part of the URL, if it exists.
    ///
    /// Example: `https://github.com/Turbo87/hosted-git-info-rs.git#rust-is-awesome` → `rust-is-awesome`
    pub fn committish(&self) -> Option<&str> {
        self.committish.as_deref()
    }

    /// The original URL type (shortcut, https, ssh, ...).
    ///
    /// Example: `https://github.com/Turbo87/hosted-git-info-rs.git` → `Https`
    pub fn default_representation(&self) -> DefaultRepresentation {
        self.default_representation
    }
}

impl str::FromStr for HostedGitInfo {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        HostedGitInfo::from_url(s)
    }
}

// look for github shorthand inputs, such as npm/cli
fn is_github_shorthand(arg: &str) -> bool {
    // it cannot contain whitespace before the first #
    // it cannot start with a / because that's probably an absolute file path
    // but it must include a slash since repos are username/repository
    // it cannot start with a . because that's probably a relative file path
    // it cannot start with an @ because that's a scoped package if it passes the other tests
    // it cannot contain a : before a # because that tells us that there's a protocol
    // a second / may not exist before a #

    // const firstHash = arg.indexOf('#')
    let first_hash = arg.find('#');
    // const firstSlash = arg.indexOf('/')
    let first_slash = arg.find('/');
    // const secondSlash = arg.indexOf('/', firstSlash + 1)
    let second_slash = first_slash.and_then(|first_slash| arg[first_slash + 1..].find('/'));
    // const firstColon = arg.indexOf(':')
    let first_colon = arg.find(':');
    // const firstSpace = /\s/.exec(arg)
    let first_space = arg.find(char::is_whitespace);
    // const firstAt = arg.indexOf('@')
    let first_at = arg.find('@');

    // const spaceOnlyAfterHash = !firstSpace || (firstHash > -1 && firstSpace.index > firstHash)
    let space_only_after_hash =
        first_space.is_none() || (first_hash.is_some() && first_space > first_hash);
    // const atOnlyAfterHash = firstAt === -1 || (firstHash > -1 && firstAt > firstHash)
    let at_only_after_hash = first_at.is_none() || (first_hash.is_some() && first_at > first_hash);
    // const colonOnlyAfterHash = firstColon === -1 || (firstHash > -1 && firstColon > firstHash)
    let colon_only_after_hash =
        first_colon.is_none() || (first_hash.is_some() && first_colon > first_hash);
    // const secondSlashOnlyAfterHash = secondSlash === -1 || (firstHash > -1 && secondSlash > firstHash)
    let second_slash_only_after_hash =
        second_slash.is_none() || (first_hash.is_some() && second_slash > first_hash);
    // const hasSlash = firstSlash > 0
    let has_slash = matches!(first_slash, Some(first_slash) if first_slash > 0);

    // if a # is found, what we really want to know is that the character immediately before # is not a /

    // const doesNotEndWithSlash = firstHash > -1 ? arg[firstHash - 1] !== '/' : !arg.endsWith('/')
    let does_not_end_with_slash = if let Some(first_hash) = first_hash {
        arg.as_bytes().get(first_hash - 1) != Some(&b'/')
    } else {
        !arg.ends_with('/')
    };
    // const doesNotStartWithDot = !arg.startsWith('.')
    let does_not_start_with_dot = !arg.starts_with('.');

    // return spaceOnlyAfterHash && hasSlash && doesNotEndWithSlash && doesNotStartWithDot && atOnlyAfterHash && colonOnlyAfterHash && secondSlashOnlyAfterHash
    space_only_after_hash
        && has_slash
        && does_not_end_with_slash
        && does_not_start_with_dot
        && at_only_after_hash
        && colon_only_after_hash
        && second_slash_only_after_hash
}

// accepts input like git:github.com:user/repo and inserts the // after the first :
fn correct_protocol(arg: &str) -> String {
    // const firstColon = arg.indexOf(':')
    if let Some(first_colon) = arg.find(':') {
        // const proto = arg.slice(0, firstColon + 1)
        let proto = &arg[0..first_colon];

        // if (knownProtocols.includes(proto)) {
        //   return arg
        // }
        if KNOWN_SCHEMES.contains(&proto) {
            return arg.to_string();
        }

        // const firstAt = arg.indexOf('@')
        // if (firstAt > -1) {
        if let Some(first_at) = arg.find('@') {
            // if (firstAt > firstColon) {
            return if first_at > first_colon {
                // return `git+ssh://${arg}`
                format!("git+ssh://{}", arg)
            } else {
                arg.to_string()
            };
        }

        // const doubleSlash = arg.indexOf('//')
        let double_slash = arg.find("//");
        // if (doubleSlash === firstColon + 1) {
        if double_slash == Some(first_colon + 1) {
            return arg.to_string();
        }

        // return arg.slice(0, firstColon + 1) + '//' + arg.slice(firstColon + 1)
        format!("{}//{}", &arg[0..first_colon + 1], &arg[first_colon + 1..])
    } else {
        arg.to_string()
    }
}

// try to parse the url as its given to us, if that throws
// then we try to clean the url and parse that result instead
fn parse_git_url(giturl: &str) -> Result<Url, url::ParseError> {
    // let result
    // try {
    //   result = new url.URL(giturl)
    // } catch (err) {}
    //
    // if (result) {
    //   return result
    // }
    Url::parse(giturl).or_else(|_error| {
        // const correctedUrl = correctUrl(giturl)
        let corrected_url = correct_url(giturl).ok_or(_error)?;

        // try {
        //   result = new url.URL(correctedUrl)
        // } catch (err) {}
        //
        // return result
        Url::parse(&corrected_url)
    })
}

// attempt to correct an scp style url so that it will parse with `new URL()`
fn correct_url(giturl: &str) -> Option<String> {
    // const firstAt = giturl.indexOf('@')
    let first_at = giturl.find('@');
    // const lastHash = giturl.lastIndexOf('#')
    let last_hash = giturl.rfind('#');
    // let firstColon = giturl.indexOf(':')
    let mut first_colon = giturl.find(':');
    // let lastColon = giturl.lastIndexOf(':', lastHash > -1 ? lastHash : Infinity)
    let last_colon = last_hash
        .map(|last_hash| &giturl[..last_hash])
        .unwrap_or(giturl)
        .rfind(':');

    // let corrected
    let mut corrected = None;

    // if (lastColon > firstAt) {
    if let Some(last_colon_) = last_colon {
        if last_colon > first_at {
            // the last : comes after the first @ (or there is no @)
            // like it would in:
            // proto://hostname.com:user/repo
            // username@hostname.com:user/repo
            // :password@hostname.com:user/repo
            // username:password@hostname.com:user/repo
            // proto://username@hostname.com:user/repo
            // proto://:password@hostname.com:user/repo
            // proto://username:password@hostname.com:user/repo
            // then we replace the last : with a / to create a valid path

            //   corrected = giturl.slice(0, lastColon) + '/' + giturl.slice(lastColon + 1)
            let corrected_ = format!("{}/{}", &giturl[0..last_colon_], &giturl[last_colon_ + 1..]);

            // and we find our new : positions

            // firstColon = corrected.indexOf(':')
            first_colon = corrected_.find(':');
            // lastColon = corrected.lastIndexOf(':')
            // last_colon = corrected_.rfind(':'); // this appears to be a bug in the original?

            corrected = Some(corrected_);
        }
    }

    // if (firstColon === -1 && giturl.indexOf('//') === -1) {
    if first_colon.is_none() && !giturl.contains("//") {
        // we have no : at all
        // as it would be in:
        // username@hostname.com/user/repo
        // then we prepend a protocol

        // corrected = `git+ssh://${corrected}`
        corrected = corrected.map(|corrected| format!("git+ssh://{}", corrected));
    }

    // return corrected
    corrected
}
