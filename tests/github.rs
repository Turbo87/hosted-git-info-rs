use claim::*;
use hosted_git_info::DefaultRepresentation::*;
use hosted_git_info::{HostedGitInfo, HostedGitInfoBuilder, Provider};

fn d() -> HostedGitInfoBuilder {
    HostedGitInfoBuilder::default()
        .provider(Provider::GitHub)
        .user("foo")
        .project("bar")
        .clone()
}

#[track_caller]
fn check(input: &str, expected: &HostedGitInfoBuilder) {
    let expected = expected.build().unwrap();
    let result = HostedGitInfo::from_url(input);
    assert_ok_eq!(result, expected, "{} does not match expectation", input);
}

#[track_caller]
fn check_err(input: &str) {
    let result = HostedGitInfo::from_url(input);
    assert_err!(result, "{} does not match expectation", input);
}

#[test]
fn extreme_shorthands() {
    // NOTE these do not accept auth at all
    check("foo/bar", d().repr(Shortcut));
    check("foo/bar#branch", d().repr(Shortcut).committish("branch"));

    check("foo/bar.git", d().repr(Shortcut));
    check(
        "foo/bar.git#branch",
        d().repr(Shortcut).committish("branch"),
    );
}

#[test]
fn shortcuts() {
    // NOTE auth is accepted but ignored
    check("github:foo/bar", d().repr(Shortcut));
    check(
        "github:foo/bar#branch",
        d().repr(Shortcut).committish("branch"),
    );
    check("github:user@foo/bar", d().repr(Shortcut));
    check(
        "github:user@foo/bar#branch",
        d().repr(Shortcut).committish("branch"),
    );
    check("github:user:password@foo/bar", d().repr(Shortcut));
    check(
        "github:user:password@foo/bar#branch",
        d().repr(Shortcut).committish("branch"),
    );
    check("github::password@foo/bar", d().repr(Shortcut));
    check(
        "github::password@foo/bar#branch",
        d().repr(Shortcut).committish("branch"),
    );

    check("github:foo/bar.git", d().repr(Shortcut));
    check(
        "github:foo/bar.git#branch",
        d().repr(Shortcut).committish("branch"),
    );
    check("github:user@foo/bar.git", d().repr(Shortcut));
    check(
        "github:user@foo/bar.git#branch",
        d().repr(Shortcut).committish("branch"),
    );
    check("github:user:password@foo/bar.git", d().repr(Shortcut));
    check(
        "github:user:password@foo/bar.git#branch",
        d().repr(Shortcut).committish("branch"),
    );
    check("github::password@foo/bar.git", d().repr(Shortcut));
    check(
        "github::password@foo/bar.git#branch",
        d().repr(Shortcut).committish("branch"),
    );
}

#[test]
fn git_urls() {
    // NOTE auth is accepted and respected
    check("git://github.com/foo/bar", d().repr(Git));
    check(
        "git://github.com/foo/bar#branch",
        d().repr(Git).committish("branch"),
    );
    check("git://user@github.com/foo/bar", d().repr(Git).auth("user"));
    check(
        "git://user@github.com/foo/bar#branch",
        d().repr(Git).auth("user").committish("branch"),
    );
    check(
        "git://user:password@github.com/foo/bar",
        d().repr(Git).auth("user:password"),
    );
    check(
        "git://user:password@github.com/foo/bar#branch",
        d().repr(Git).auth("user:password").committish("branch"),
    );
    check(
        "git://:password@github.com/foo/bar",
        d().repr(Git).auth(":password"),
    );
    check(
        "git://:password@github.com/foo/bar#branch",
        d().repr(Git).auth(":password").committish("branch"),
    );

    check("git://github.com/foo/bar.git", d().repr(Git));
    check(
        "git://github.com/foo/bar.git#branch",
        d().repr(Git).committish("branch"),
    );
    check(
        "git://git@github.com/foo/bar.git",
        d().repr(Git).auth("git"),
    );
    check(
        "git://git@github.com/foo/bar.git#branch",
        d().repr(Git).auth("git").committish("branch"),
    );
    check(
        "git://user:password@github.com/foo/bar.git",
        d().repr(Git).auth("user:password"),
    );
    check(
        "git://user:password@github.com/foo/bar.git#branch",
        d().repr(Git).auth("user:password").committish("branch"),
    );
    check(
        "git://:password@github.com/foo/bar.git",
        d().repr(Git).auth(":password"),
    );
    check(
        "git://:password@github.com/foo/bar.git#branch",
        d().repr(Git).auth(":password").committish("branch"),
    );
}

#[test]
fn no_protocol() {
    // NOTE auth is _required_ (see invalid list) but ignored
    check("user@github.com:foo/bar", d().repr(Ssh));
    check(
        "user@github.com:foo/bar#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("user:password@github.com:foo/bar", d().repr(Ssh));
    check(
        "user:password@github.com:foo/bar#branch",
        d().repr(Ssh).committish("branch"),
    );
    check(":password@github.com:foo/bar", d().repr(Ssh));
    check(
        ":password@github.com:foo/bar#branch",
        d().repr(Ssh).committish("branch"),
    );

    check("user@github.com:foo/bar.git", d().repr(Ssh));
    check(
        "user@github.com:foo/bar.git#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("user:password@github.com:foo/bar.git", d().repr(Ssh));
    check(
        "user:password@github.com:foo/bar.git#branch",
        d().repr(Ssh).committish("branch"),
    );
    check(":password@github.com:foo/bar.git", d().repr(Ssh));
    check(
        ":password@github.com:foo/bar.git#branch",
        d().repr(Ssh).committish("branch"),
    );
}

#[test]
fn git_ssh_urls() {
    // NOTE auth is accepted but ignored
    check("git+ssh://github.com:foo/bar", d().repr(Ssh));
    check(
        "git+ssh://github.com:foo/bar#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("git+ssh://user@github.com:foo/bar", d().repr(Ssh));
    check(
        "git+ssh://user@github.com:foo/bar#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("git+ssh://user:password@github.com:foo/bar", d().repr(Ssh));
    check(
        "git+ssh://user:password@github.com:foo/bar#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("git+ssh://:password@github.com:foo/bar", d().repr(Ssh));
    check(
        "git+ssh://:password@github.com:foo/bar#branch",
        d().repr(Ssh).committish("branch"),
    );

    check("git+ssh://github.com:foo/bar.git", d().repr(Ssh));
    check(
        "git+ssh://github.com:foo/bar.git#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("git+ssh://user@github.com:foo/bar.git", d().repr(Ssh));
    check(
        "git+ssh://user@github.com:foo/bar.git#branch",
        d().repr(Ssh).committish("branch"),
    );
    check(
        "git+ssh://user:password@github.com:foo/bar.git",
        d().repr(Ssh),
    );
    check(
        "git+ssh://user:password@github.com:foo/bar.git#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("git+ssh://:password@github.com:foo/bar.git", d().repr(Ssh));
    check(
        "git+ssh://:password@github.com:foo/bar.git#branch",
        d().repr(Ssh).committish("branch"),
    );
}

#[test]
fn ssh_urls() {
    // NOTE auth is accepted but ignored
    check("ssh://github.com:foo/bar", d().repr(Ssh));
    check(
        "ssh://github.com:foo/bar#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("ssh://user@github.com:foo/bar", d().repr(Ssh));
    check(
        "ssh://user@github.com:foo/bar#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("ssh://user:password@github.com:foo/bar", d().repr(Ssh));
    check(
        "ssh://user:password@github.com:foo/bar#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("ssh://:password@github.com:foo/bar", d().repr(Ssh));
    check(
        "ssh://:password@github.com:foo/bar#branch",
        d().repr(Ssh).committish("branch"),
    );

    check("ssh://github.com:foo/bar.git", d().repr(Ssh));
    check(
        "ssh://github.com:foo/bar.git#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("ssh://user@github.com:foo/bar.git", d().repr(Ssh));
    check(
        "ssh://user@github.com:foo/bar.git#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("ssh://user:password@github.com:foo/bar.git", d().repr(Ssh));
    check(
        "ssh://user:password@github.com:foo/bar.git#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("ssh://:password@github.com:foo/bar.git", d().repr(Ssh));
    check(
        "ssh://:password@github.com:foo/bar.git#branch",
        d().repr(Ssh).committish("branch"),
    );
}

#[test]
fn git_https_urls() {
    // NOTE auth is accepted and respected
    check("git+https://github.com/foo/bar", d().repr(Https));
    check(
        "git+https://github.com/foo/bar#branch",
        d().repr(Https).committish("branch"),
    );
    check(
        "git+https://user@github.com/foo/bar",
        d().repr(Https).auth("user"),
    );
    check(
        "git+https://user@github.com/foo/bar#branch",
        d().repr(Https).auth("user").committish("branch"),
    );
    check(
        "git+https://user:password@github.com/foo/bar",
        d().repr(Https).auth("user:password"),
    );
    check(
        "git+https://user:password@github.com/foo/bar#branch",
        d().repr(Https).auth("user:password").committish("branch"),
    );
    check(
        "git+https://:password@github.com/foo/bar",
        d().repr(Https).auth(":password"),
    );
    check(
        "git+https://:password@github.com/foo/bar#branch",
        d().repr(Https).auth(":password").committish("branch"),
    );

    check("git+https://github.com/foo/bar.git", d().repr(Https));
    check(
        "git+https://github.com/foo/bar.git#branch",
        d().repr(Https).committish("branch"),
    );
    check(
        "git+https://user@github.com/foo/bar.git",
        d().repr(Https).auth("user"),
    );
    check(
        "git+https://user@github.com/foo/bar.git#branch",
        d().repr(Https).auth("user").committish("branch"),
    );
    check(
        "git+https://user:password@github.com/foo/bar.git",
        d().repr(Https).auth("user:password"),
    );
    check(
        "git+https://user:password@github.com/foo/bar.git#branch",
        d().repr(Https).auth("user:password").committish("branch"),
    );
    check(
        "git+https://:password@github.com/foo/bar.git",
        d().repr(Https).auth(":password"),
    );
    check(
        "git+https://:password@github.com/foo/bar.git#branch",
        d().repr(Https).auth(":password").committish("branch"),
    );
}

#[test]
fn https_urls() {
    // // NOTE auth is accepted and respected
    check("https://github.com/foo/bar", d().repr(Https));
    check(
        "https://github.com/foo/bar#branch",
        d().repr(Https).committish("branch"),
    );
    check(
        "https://user@github.com/foo/bar",
        d().repr(Https).auth("user"),
    );
    check(
        "https://user@github.com/foo/bar#branch",
        d().repr(Https).auth("user").committish("branch"),
    );
    check(
        "https://user:password@github.com/foo/bar",
        d().repr(Https).auth("user:password"),
    );
    check(
        "https://user:password@github.com/foo/bar#branch",
        d().repr(Https).auth("user:password").committish("branch"),
    );
    check(
        "https://:password@github.com/foo/bar",
        d().repr(Https).auth(":password"),
    );
    check(
        "https://:password@github.com/foo/bar#branch",
        d().repr(Https).auth(":password").committish("branch"),
    );

    check("https://github.com/foo/bar.git", d().repr(Https));
    check(
        "https://github.com/foo/bar.git#branch",
        d().repr(Https).committish("branch"),
    );
    check(
        "https://user@github.com/foo/bar.git",
        d().repr(Https).auth("user"),
    );
    check(
        "https://user@github.com/foo/bar.git#branch",
        d().repr(Https).auth("user").committish("branch"),
    );
    check(
        "https://user:password@github.com/foo/bar.git",
        d().repr(Https).auth("user:password"),
    );
    check(
        "https://user:password@github.com/foo/bar.git#branch",
        d().repr(Https).auth("user:password").committish("branch"),
    );
    check(
        "https://:password@github.com/foo/bar.git",
        d().repr(Https).auth(":password"),
    );
    check(
        "https://:password@github.com/foo/bar.git#branch",
        d().repr(Https).auth(":password").committish("branch"),
    );
}
#[test]
fn almost_garbage() {
    // inputs that are not quite proper but we accept anyway
    check("https://www.github.com/foo/bar", d().repr(Https));
    check(
        "foo/bar#branch with space",
        d().repr(Shortcut).committish("branch with space"),
    );
    check(
        "https://github.com/foo/bar/tree/branch",
        d().repr(Https).committish("branch"),
    );
    check(
        "user..blerg--/..foo-js# . . . . . some . tags / / /",
        d().repr(Shortcut)
            .user("user..blerg--")
            .project("..foo-js")
            .committish(" . . . . . some . tags / / /"),
    );
}

#[test]
fn invalid() {
    // foo/bar shorthand but specifying auth
    check_err("user@foo/bar");
    check_err("user:password@foo/bar");
    check_err(":password@foo/bar");
    // foo/bar shorthand but with a space in it
    check_err("foo/ bar");
    // string that ends with a slash, probably a directory
    check_err("foo/bar/");
    // git@github.com style, but omitting the username
    check_err("github.com:foo/bar");
    check_err("github.com/foo/bar");
    // invalid URI encoding
    check_err("github:foo%ff/bar");
    // missing path
    check_err("git+ssh://git@github.com:");
    // a deep url to something we don't know
    check_err("https://github.com/foo/bar/issues");
}
