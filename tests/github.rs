use claim::*;
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
    check("foo/bar", &d());
    check("foo/bar#branch", d().committish("branch"));

    check("foo/bar.git", &d());
    check("foo/bar.git#branch", d().committish("branch"));
}

#[test]
fn shortcuts() {
    // NOTE auth is accepted but ignored
    check("github:foo/bar", &d());
    check("github:foo/bar#branch", d().committish("branch"));
    check("github:user@foo/bar", &d());
    check("github:user@foo/bar#branch", d().committish("branch"));
    check("github:user:password@foo/bar", &d());
    check(
        "github:user:password@foo/bar#branch",
        d().committish("branch"),
    );
    check("github::password@foo/bar", &d());
    check("github::password@foo/bar#branch", d().committish("branch"));

    check("github:foo/bar.git", &d());
    check("github:foo/bar.git#branch", d().committish("branch"));
    check("github:user@foo/bar.git", &d());
    check("github:user@foo/bar.git#branch", d().committish("branch"));
    check("github:user:password@foo/bar.git", &d());
    check(
        "github:user:password@foo/bar.git#branch",
        d().committish("branch"),
    );
    check("github::password@foo/bar.git", &d());
    check(
        "github::password@foo/bar.git#branch",
        d().committish("branch"),
    );
}

#[test]
fn git_urls() {
    // NOTE auth is accepted and respected
    check("git://github.com/foo/bar", &d());
    check("git://github.com/foo/bar#branch", d().committish("branch"));
    check("git://user@github.com/foo/bar", d().auth("user"));
    check(
        "git://user@github.com/foo/bar#branch",
        d().auth("user").committish("branch"),
    );
    check(
        "git://user:password@github.com/foo/bar",
        d().auth("user:password"),
    );
    check(
        "git://user:password@github.com/foo/bar#branch",
        d().auth("user:password").committish("branch"),
    );
    check("git://:password@github.com/foo/bar", d().auth(":password"));
    check(
        "git://:password@github.com/foo/bar#branch",
        d().auth(":password").committish("branch"),
    );

    check("git://github.com/foo/bar.git", &d());
    check(
        "git://github.com/foo/bar.git#branch",
        d().committish("branch"),
    );
    check("git://git@github.com/foo/bar.git", d().auth("git"));
    check(
        "git://git@github.com/foo/bar.git#branch",
        d().auth("git").committish("branch"),
    );
    check(
        "git://user:password@github.com/foo/bar.git",
        d().auth("user:password"),
    );
    check(
        "git://user:password@github.com/foo/bar.git#branch",
        d().auth("user:password").committish("branch"),
    );
    check(
        "git://:password@github.com/foo/bar.git",
        d().auth(":password"),
    );
    check(
        "git://:password@github.com/foo/bar.git#branch",
        d().auth(":password").committish("branch"),
    );
}

#[test]
fn no_protocol() {
    // NOTE auth is _required_ (see invalid list) but ignored
    check("user@github.com:foo/bar", &d());
    check("user@github.com:foo/bar#branch", d().committish("branch"));
    check("user:password@github.com:foo/bar", &d());
    check(
        "user:password@github.com:foo/bar#branch",
        d().committish("branch"),
    );
    check(":password@github.com:foo/bar", &d());
    check(
        ":password@github.com:foo/bar#branch",
        d().committish("branch"),
    );

    check("user@github.com:foo/bar.git", &d());
    check(
        "user@github.com:foo/bar.git#branch",
        d().committish("branch"),
    );
    check("user:password@github.com:foo/bar.git", &d());
    check(
        "user:password@github.com:foo/bar.git#branch",
        d().committish("branch"),
    );
    check(":password@github.com:foo/bar.git", &d());
    check(
        ":password@github.com:foo/bar.git#branch",
        d().committish("branch"),
    );
}

#[test]
fn git_ssh_urls() {
    // NOTE auth is accepted but ignored
    check("git+ssh://github.com:foo/bar", &d());
    check(
        "git+ssh://github.com:foo/bar#branch",
        d().committish("branch"),
    );
    check("git+ssh://user@github.com:foo/bar", &d());
    check(
        "git+ssh://user@github.com:foo/bar#branch",
        d().committish("branch"),
    );
    check("git+ssh://user:password@github.com:foo/bar", &d());
    check(
        "git+ssh://user:password@github.com:foo/bar#branch",
        d().committish("branch"),
    );
    check("git+ssh://:password@github.com:foo/bar", &d());
    check(
        "git+ssh://:password@github.com:foo/bar#branch",
        d().committish("branch"),
    );

    check("git+ssh://github.com:foo/bar.git", &d());
    check(
        "git+ssh://github.com:foo/bar.git#branch",
        d().committish("branch"),
    );
    check("git+ssh://user@github.com:foo/bar.git", &d());
    check(
        "git+ssh://user@github.com:foo/bar.git#branch",
        d().committish("branch"),
    );
    check("git+ssh://user:password@github.com:foo/bar.git", &d());
    check(
        "git+ssh://user:password@github.com:foo/bar.git#branch",
        d().committish("branch"),
    );
    check("git+ssh://:password@github.com:foo/bar.git", &d());
    check(
        "git+ssh://:password@github.com:foo/bar.git#branch",
        d().committish("branch"),
    );
}

#[test]
fn ssh_urls() {
    // NOTE auth is accepted but ignored
    check("ssh://github.com:foo/bar", &d());
    check("ssh://github.com:foo/bar#branch", d().committish("branch"));
    check("ssh://user@github.com:foo/bar", &d());
    check(
        "ssh://user@github.com:foo/bar#branch",
        d().committish("branch"),
    );
    check("ssh://user:password@github.com:foo/bar", &d());
    check(
        "ssh://user:password@github.com:foo/bar#branch",
        d().committish("branch"),
    );
    check("ssh://:password@github.com:foo/bar", &d());
    check(
        "ssh://:password@github.com:foo/bar#branch",
        d().committish("branch"),
    );

    check("ssh://github.com:foo/bar.git", &d());
    check(
        "ssh://github.com:foo/bar.git#branch",
        d().committish("branch"),
    );
    check("ssh://user@github.com:foo/bar.git", &d());
    check(
        "ssh://user@github.com:foo/bar.git#branch",
        d().committish("branch"),
    );
    check("ssh://user:password@github.com:foo/bar.git", &d());
    check(
        "ssh://user:password@github.com:foo/bar.git#branch",
        d().committish("branch"),
    );
    check("ssh://:password@github.com:foo/bar.git", &d());
    check(
        "ssh://:password@github.com:foo/bar.git#branch",
        d().committish("branch"),
    );
}

#[test]
fn git_https_urls() {
    // NOTE auth is accepted and respected
    check("git+https://github.com/foo/bar", &d());
    check(
        "git+https://github.com/foo/bar#branch",
        d().committish("branch"),
    );
    check("git+https://user@github.com/foo/bar", d().auth("user"));
    check(
        "git+https://user@github.com/foo/bar#branch",
        d().auth("user").committish("branch"),
    );
    check(
        "git+https://user:password@github.com/foo/bar",
        d().auth("user:password"),
    );
    check(
        "git+https://user:password@github.com/foo/bar#branch",
        d().auth("user:password").committish("branch"),
    );
    check(
        "git+https://:password@github.com/foo/bar",
        d().auth(":password"),
    );
    check(
        "git+https://:password@github.com/foo/bar#branch",
        d().auth(":password").committish("branch"),
    );

    check("git+https://github.com/foo/bar.git", &d());
    check(
        "git+https://github.com/foo/bar.git#branch",
        d().committish("branch"),
    );
    check("git+https://user@github.com/foo/bar.git", d().auth("user"));
    check(
        "git+https://user@github.com/foo/bar.git#branch",
        d().auth("user").committish("branch"),
    );
    check(
        "git+https://user:password@github.com/foo/bar.git",
        d().auth("user:password"),
    );
    check(
        "git+https://user:password@github.com/foo/bar.git#branch",
        d().auth("user:password").committish("branch"),
    );
    check(
        "git+https://:password@github.com/foo/bar.git",
        d().auth(":password"),
    );
    check(
        "git+https://:password@github.com/foo/bar.git#branch",
        d().auth(":password").committish("branch"),
    );
}

#[test]
fn https_urls() {
    // // NOTE auth is accepted and respected
    check("https://github.com/foo/bar", &d());
    check(
        "https://github.com/foo/bar#branch",
        d().committish("branch"),
    );
    check("https://user@github.com/foo/bar", d().auth("user"));
    check(
        "https://user@github.com/foo/bar#branch",
        d().auth("user").committish("branch"),
    );
    check(
        "https://user:password@github.com/foo/bar",
        d().auth("user:password"),
    );
    check(
        "https://user:password@github.com/foo/bar#branch",
        d().auth("user:password").committish("branch"),
    );
    check(
        "https://:password@github.com/foo/bar",
        d().auth(":password"),
    );
    check(
        "https://:password@github.com/foo/bar#branch",
        d().auth(":password").committish("branch"),
    );

    check("https://github.com/foo/bar.git", &d());
    check(
        "https://github.com/foo/bar.git#branch",
        d().committish("branch"),
    );
    check("https://user@github.com/foo/bar.git", d().auth("user"));
    check(
        "https://user@github.com/foo/bar.git#branch",
        d().auth("user").committish("branch"),
    );
    check(
        "https://user:password@github.com/foo/bar.git",
        d().auth("user:password"),
    );
    check(
        "https://user:password@github.com/foo/bar.git#branch",
        d().auth("user:password").committish("branch"),
    );
    check(
        "https://:password@github.com/foo/bar.git",
        d().auth(":password"),
    );
    check(
        "https://:password@github.com/foo/bar.git#branch",
        d().auth(":password").committish("branch"),
    );
}
#[test]
fn almost_garbage() {
    // inputs that are not quite proper but we accept anyway
    check("https://www.github.com/foo/bar", &d());
    check(
        "foo/bar#branch with space",
        d().committish("branch with space"),
    );
    check(
        "https://github.com/foo/bar/tree/branch",
        d().committish("branch"),
    );
    check(
        "user..blerg--/..foo-js# . . . . . some . tags / / /",
        d().user("user..blerg--")
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
