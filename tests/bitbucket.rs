use claim::*;
use hosted_git_info::{HostedGitInfo, HostedGitInfoBuilder, Provider};

fn d() -> HostedGitInfoBuilder {
    HostedGitInfoBuilder::default()
        .provider(Provider::BitBucket)
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
fn shortcuts() {
    // NOTE auth is accepted but ignored
    check("bitbucket:foo/bar", &d());
    check("bitbucket:foo/bar#branch", d().committish("branch"));
    check("bitbucket:user@foo/bar", &d());
    check("bitbucket:user@foo/bar#branch", d().committish("branch"));
    check("bitbucket:user:password@foo/bar", &d());
    check(
        "bitbucket:user:password@foo/bar#branch",
        d().committish("branch"),
    );
    check("bitbucket::password@foo/bar", &d());
    check(
        "bitbucket::password@foo/bar#branch",
        d().committish("branch"),
    );

    check("bitbucket:foo/bar.git", &d());
    check("bitbucket:foo/bar.git#branch", d().committish("branch"));
    check("bitbucket:user@foo/bar.git", &d());
    check(
        "bitbucket:user@foo/bar.git#branch",
        d().committish("branch"),
    );
    check("bitbucket:user:password@foo/bar.git", &d());
    check(
        "bitbucket:user:password@foo/bar.git#branch",
        d().committish("branch"),
    );
    check("bitbucket::password@foo/bar.git", &d());
    check(
        "bitbucket::password@foo/bar.git#branch",
        d().committish("branch"),
    );
}

#[test]
fn no_protocol() {
    // NOTE auth is accepted but ignored
    check("git@bitbucket.org:foo/bar", &d());
    check("git@bitbucket.org:foo/bar#branch", d().committish("branch"));
    check("user@bitbucket.org:foo/bar", &d());
    check(
        "user@bitbucket.org:foo/bar#branch",
        d().committish("branch"),
    );
    check("user:password@bitbucket.org:foo/bar", &d());
    check(
        "user:password@bitbucket.org:foo/bar#branch",
        d().committish("branch"),
    );
    check(":password@bitbucket.org:foo/bar", &d());
    check(
        ":password@bitbucket.org:foo/bar#branch",
        d().committish("branch"),
    );

    check("git@bitbucket.org:foo/bar.git", &d());
    check(
        "git@bitbucket.org:foo/bar.git#branch",
        d().committish("branch"),
    );
    check("user@bitbucket.org:foo/bar.git", &d());
    check(
        "user@bitbucket.org:foo/bar.git#branch",
        d().committish("branch"),
    );
    check("user:password@bitbucket.org:foo/bar.git", &d());
    check(
        "user:password@bitbucket.org:foo/bar.git#branch",
        d().committish("branch"),
    );
    check(":password@bitbucket.org:foo/bar.git", &d());
    check(
        ":password@bitbucket.org:foo/bar.git#branch",
        d().committish("branch"),
    );
}

#[test]
fn git_ssh_urls() {
    // NOTE auth is accepted but ignored
    check("git+ssh://bitbucket.org:foo/bar", &d());
    check(
        "git+ssh://bitbucket.org:foo/bar#branch",
        d().committish("branch"),
    );
    check("git+ssh://user@bitbucket.org:foo/bar", &d());
    check(
        "git+ssh://user@bitbucket.org:foo/bar#branch",
        d().committish("branch"),
    );
    check("git+ssh://user:password@bitbucket.org:foo/bar", &d());
    check(
        "git+ssh://user:password@bitbucket.org:foo/bar#branch",
        d().committish("branch"),
    );
    check("git+ssh://:password@bitbucket.org:foo/bar", &d());
    check(
        "git+ssh://:password@bitbucket.org:foo/bar#branch",
        d().committish("branch"),
    );

    check("git+ssh://bitbucket.org:foo/bar.git", &d());
    check(
        "git+ssh://bitbucket.org:foo/bar.git#branch",
        d().committish("branch"),
    );
    check("git+ssh://user@bitbucket.org:foo/bar.git", &d());
    check(
        "git+ssh://user@bitbucket.org:foo/bar.git#branch",
        d().committish("branch"),
    );
    check("git+ssh://user:password@bitbucket.org:foo/bar.git", &d());
    check(
        "git+ssh://user:password@bitbucket.org:foo/bar.git#branch",
        d().committish("branch"),
    );
    check("git+ssh://:password@bitbucket.org:foo/bar.git", &d());
    check(
        "git+ssh://:password@bitbucket.org:foo/bar.git#branch",
        d().committish("branch"),
    );
}

#[test]
fn ssh_urls() {
    // NOTE auth is accepted but ignored
    check("ssh://bitbucket.org:foo/bar", &d());
    check(
        "ssh://bitbucket.org:foo/bar#branch",
        d().committish("branch"),
    );
    check("ssh://user@bitbucket.org:foo/bar", &d());
    check(
        "ssh://user@bitbucket.org:foo/bar#branch",
        d().committish("branch"),
    );
    check("ssh://user:password@bitbucket.org:foo/bar", &d());
    check(
        "ssh://user:password@bitbucket.org:foo/bar#branch",
        d().committish("branch"),
    );
    check("ssh://:password@bitbucket.org:foo/bar", &d());
    check(
        "ssh://:password@bitbucket.org:foo/bar#branch",
        d().committish("branch"),
    );

    check("ssh://bitbucket.org:foo/bar.git", &d());
    check(
        "ssh://bitbucket.org:foo/bar.git#branch",
        d().committish("branch"),
    );
    check("ssh://user@bitbucket.org:foo/bar.git", &d());
    check(
        "ssh://user@bitbucket.org:foo/bar.git#branch",
        d().committish("branch"),
    );
    check("ssh://user:password@bitbucket.org:foo/bar.git", &d());
    check(
        "ssh://user:password@bitbucket.org:foo/bar.git#branch",
        d().committish("branch"),
    );
    check("ssh://:password@bitbucket.org:foo/bar.git", &d());
    check(
        "ssh://:password@bitbucket.org:foo/bar.git#branch",
        d().committish("branch"),
    );
}

#[test]
fn git_https_urls() {
    // NOTE auth is accepted and respected
    check("git+https://bitbucket.org/foo/bar", &d());
    check(
        "git+https://bitbucket.org/foo/bar#branch",
        d().committish("branch"),
    );
    check("git+https://user@bitbucket.org/foo/bar", d().auth("user"));
    check(
        "git+https://user@bitbucket.org/foo/bar#branch",
        d().auth("user").committish("branch"),
    );
    check(
        "git+https://user:password@bitbucket.org/foo/bar",
        d().auth("user:password"),
    );
    check(
        "git+https://user:password@bitbucket.org/foo/bar#branch",
        d().auth("user:password").committish("branch"),
    );
    check(
        "git+https://:password@bitbucket.org/foo/bar",
        d().auth(":password"),
    );
    check(
        "git+https://:password@bitbucket.org/foo/bar#branch",
        d().auth(":password").committish("branch"),
    );

    check("git+https://bitbucket.org/foo/bar.git", &d());
    check(
        "git+https://bitbucket.org/foo/bar.git#branch",
        d().committish("branch"),
    );
    check(
        "git+https://user@bitbucket.org/foo/bar.git",
        d().auth("user"),
    );
    check(
        "git+https://user@bitbucket.org/foo/bar.git#branch",
        d().auth("user").committish("branch"),
    );
    check(
        "git+https://user:password@bitbucket.org/foo/bar.git",
        d().auth("user:password"),
    );
    check(
        "git+https://user:password@bitbucket.org/foo/bar.git#branch",
        d().auth("user:password").committish("branch"),
    );
    check(
        "git+https://:password@bitbucket.org/foo/bar.git",
        d().auth(":password"),
    );
    check(
        "git+https://:password@bitbucket.org/foo/bar.git#branch",
        d().auth(":password").committish("branch"),
    );
}

#[test]
fn https_urls() {
    // NOTE auth is accepted and respected
    check("https://bitbucket.org/foo/bar", &d());
    check(
        "https://bitbucket.org/foo/bar#branch",
        d().committish("branch"),
    );
    check("https://user@bitbucket.org/foo/bar", d().auth("user"));
    check(
        "https://user@bitbucket.org/foo/bar#branch",
        d().auth("user").committish("branch"),
    );
    check(
        "https://user:password@bitbucket.org/foo/bar",
        d().auth("user:password"),
    );
    check(
        "https://user:password@bitbucket.org/foo/bar#branch",
        d().auth("user:password").committish("branch"),
    );
    check(
        "https://:password@bitbucket.org/foo/bar",
        d().auth(":password"),
    );
    check(
        "https://:password@bitbucket.org/foo/bar#branch",
        d().auth(":password").committish("branch"),
    );

    check("https://bitbucket.org/foo/bar.git", &d());
    check(
        "https://bitbucket.org/foo/bar.git#branch",
        d().committish("branch"),
    );
    check("https://user@bitbucket.org/foo/bar.git", d().auth("user"));
    check(
        "https://user@bitbucket.org/foo/bar.git#branch",
        d().auth("user").committish("branch"),
    );
    check(
        "https://user:password@bitbucket.org/foo/bar.git",
        d().auth("user:password"),
    );
    check(
        "https://user:password@bitbucket.org/foo/bar.git#branch",
        d().auth("user:password").committish("branch"),
    );
    check(
        "https://:password@bitbucket.org/foo/bar.git",
        d().auth(":password"),
    );
    check(
        "https://:password@bitbucket.org/foo/bar.git#branch",
        d().auth(":password").committish("branch"),
    );
}

#[test]
fn invalid() {
    // invalid protocol
    check_err("git://bitbucket.org/foo/bar");
    // url to get a tarball
    check_err("https://bitbucket.org/foo/bar/get/archive.tar.gz");
    // missing project
    check_err("https://bitbucket.org/foo");
}
