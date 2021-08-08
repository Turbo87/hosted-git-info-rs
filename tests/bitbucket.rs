use claim::*;
use hosted_git_info::DefaultRepresentation::*;
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
    check("bitbucket:foo/bar", d().repr(Shortcut));
    check(
        "bitbucket:foo/bar#branch",
        d().repr(Shortcut).committish("branch"),
    );
    check("bitbucket:user@foo/bar", d().repr(Shortcut));
    check(
        "bitbucket:user@foo/bar#branch",
        d().repr(Shortcut).committish("branch"),
    );
    check("bitbucket:user:password@foo/bar", d().repr(Shortcut));
    check(
        "bitbucket:user:password@foo/bar#branch",
        d().repr(Shortcut).committish("branch"),
    );
    check("bitbucket::password@foo/bar", d().repr(Shortcut));
    check(
        "bitbucket::password@foo/bar#branch",
        d().repr(Shortcut).committish("branch"),
    );

    check("bitbucket:foo/bar.git", d().repr(Shortcut));
    check(
        "bitbucket:foo/bar.git#branch",
        d().repr(Shortcut).committish("branch"),
    );
    check("bitbucket:user@foo/bar.git", d().repr(Shortcut));
    check(
        "bitbucket:user@foo/bar.git#branch",
        d().repr(Shortcut).committish("branch"),
    );
    check("bitbucket:user:password@foo/bar.git", d().repr(Shortcut));
    check(
        "bitbucket:user:password@foo/bar.git#branch",
        d().repr(Shortcut).committish("branch"),
    );
    check("bitbucket::password@foo/bar.git", d().repr(Shortcut));
    check(
        "bitbucket::password@foo/bar.git#branch",
        d().repr(Shortcut).committish("branch"),
    );
}

#[test]
fn no_protocol() {
    // NOTE auth is accepted but ignored
    check("git@bitbucket.org:foo/bar", d().repr(Ssh));
    check(
        "git@bitbucket.org:foo/bar#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("user@bitbucket.org:foo/bar", d().repr(Ssh));
    check(
        "user@bitbucket.org:foo/bar#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("user:password@bitbucket.org:foo/bar", d().repr(Ssh));
    check(
        "user:password@bitbucket.org:foo/bar#branch",
        d().repr(Ssh).committish("branch"),
    );
    check(":password@bitbucket.org:foo/bar", d().repr(Ssh));
    check(
        ":password@bitbucket.org:foo/bar#branch",
        d().repr(Ssh).committish("branch"),
    );

    check("git@bitbucket.org:foo/bar.git", d().repr(Ssh));
    check(
        "git@bitbucket.org:foo/bar.git#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("user@bitbucket.org:foo/bar.git", d().repr(Ssh));
    check(
        "user@bitbucket.org:foo/bar.git#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("user:password@bitbucket.org:foo/bar.git", d().repr(Ssh));
    check(
        "user:password@bitbucket.org:foo/bar.git#branch",
        d().repr(Ssh).committish("branch"),
    );
    check(":password@bitbucket.org:foo/bar.git", d().repr(Ssh));
    check(
        ":password@bitbucket.org:foo/bar.git#branch",
        d().repr(Ssh).committish("branch"),
    );
}

#[test]
fn git_ssh_urls() {
    // NOTE auth is accepted but ignored
    check("git+ssh://bitbucket.org:foo/bar", d().repr(Ssh));
    check(
        "git+ssh://bitbucket.org:foo/bar#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("git+ssh://user@bitbucket.org:foo/bar", d().repr(Ssh));
    check(
        "git+ssh://user@bitbucket.org:foo/bar#branch",
        d().repr(Ssh).committish("branch"),
    );
    check(
        "git+ssh://user:password@bitbucket.org:foo/bar",
        d().repr(Ssh),
    );
    check(
        "git+ssh://user:password@bitbucket.org:foo/bar#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("git+ssh://:password@bitbucket.org:foo/bar", d().repr(Ssh));
    check(
        "git+ssh://:password@bitbucket.org:foo/bar#branch",
        d().repr(Ssh).committish("branch"),
    );

    check("git+ssh://bitbucket.org:foo/bar.git", d().repr(Ssh));
    check(
        "git+ssh://bitbucket.org:foo/bar.git#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("git+ssh://user@bitbucket.org:foo/bar.git", d().repr(Ssh));
    check(
        "git+ssh://user@bitbucket.org:foo/bar.git#branch",
        d().repr(Ssh).committish("branch"),
    );
    check(
        "git+ssh://user:password@bitbucket.org:foo/bar.git",
        d().repr(Ssh),
    );
    check(
        "git+ssh://user:password@bitbucket.org:foo/bar.git#branch",
        d().repr(Ssh).committish("branch"),
    );
    check(
        "git+ssh://:password@bitbucket.org:foo/bar.git",
        d().repr(Ssh),
    );
    check(
        "git+ssh://:password@bitbucket.org:foo/bar.git#branch",
        d().repr(Ssh).committish("branch"),
    );
}

#[test]
fn ssh_urls() {
    // NOTE auth is accepted but ignored
    check("ssh://bitbucket.org:foo/bar", d().repr(Ssh));
    check(
        "ssh://bitbucket.org:foo/bar#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("ssh://user@bitbucket.org:foo/bar", d().repr(Ssh));
    check(
        "ssh://user@bitbucket.org:foo/bar#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("ssh://user:password@bitbucket.org:foo/bar", d().repr(Ssh));
    check(
        "ssh://user:password@bitbucket.org:foo/bar#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("ssh://:password@bitbucket.org:foo/bar", d().repr(Ssh));
    check(
        "ssh://:password@bitbucket.org:foo/bar#branch",
        d().repr(Ssh).committish("branch"),
    );

    check("ssh://bitbucket.org:foo/bar.git", d().repr(Ssh));
    check(
        "ssh://bitbucket.org:foo/bar.git#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("ssh://user@bitbucket.org:foo/bar.git", d().repr(Ssh));
    check(
        "ssh://user@bitbucket.org:foo/bar.git#branch",
        d().repr(Ssh).committish("branch"),
    );
    check(
        "ssh://user:password@bitbucket.org:foo/bar.git",
        d().repr(Ssh),
    );
    check(
        "ssh://user:password@bitbucket.org:foo/bar.git#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("ssh://:password@bitbucket.org:foo/bar.git", d().repr(Ssh));
    check(
        "ssh://:password@bitbucket.org:foo/bar.git#branch",
        d().repr(Ssh).committish("branch"),
    );
}

#[test]
fn git_https_urls() {
    // NOTE auth is accepted and respected
    check("git+https://bitbucket.org/foo/bar", d().repr(Https));
    check(
        "git+https://bitbucket.org/foo/bar#branch",
        d().repr(Https).committish("branch"),
    );
    check(
        "git+https://user@bitbucket.org/foo/bar",
        d().repr(Https).auth("user"),
    );
    check(
        "git+https://user@bitbucket.org/foo/bar#branch",
        d().repr(Https).auth("user").committish("branch"),
    );
    check(
        "git+https://user:password@bitbucket.org/foo/bar",
        d().repr(Https).auth("user:password"),
    );
    check(
        "git+https://user:password@bitbucket.org/foo/bar#branch",
        d().repr(Https).auth("user:password").committish("branch"),
    );
    check(
        "git+https://:password@bitbucket.org/foo/bar",
        d().repr(Https).auth(":password"),
    );
    check(
        "git+https://:password@bitbucket.org/foo/bar#branch",
        d().repr(Https).auth(":password").committish("branch"),
    );

    check("git+https://bitbucket.org/foo/bar.git", d().repr(Https));
    check(
        "git+https://bitbucket.org/foo/bar.git#branch",
        d().repr(Https).committish("branch"),
    );
    check(
        "git+https://user@bitbucket.org/foo/bar.git",
        d().repr(Https).auth("user"),
    );
    check(
        "git+https://user@bitbucket.org/foo/bar.git#branch",
        d().repr(Https).auth("user").committish("branch"),
    );
    check(
        "git+https://user:password@bitbucket.org/foo/bar.git",
        d().repr(Https).auth("user:password"),
    );
    check(
        "git+https://user:password@bitbucket.org/foo/bar.git#branch",
        d().repr(Https).auth("user:password").committish("branch"),
    );
    check(
        "git+https://:password@bitbucket.org/foo/bar.git",
        d().repr(Https).auth(":password"),
    );
    check(
        "git+https://:password@bitbucket.org/foo/bar.git#branch",
        d().repr(Https).auth(":password").committish("branch"),
    );
}

#[test]
fn https_urls() {
    // NOTE auth is accepted and respected
    check("https://bitbucket.org/foo/bar", d().repr(Https));
    check(
        "https://bitbucket.org/foo/bar#branch",
        d().repr(Https).committish("branch"),
    );
    check(
        "https://user@bitbucket.org/foo/bar",
        d().repr(Https).auth("user"),
    );
    check(
        "https://user@bitbucket.org/foo/bar#branch",
        d().repr(Https).auth("user").committish("branch"),
    );
    check(
        "https://user:password@bitbucket.org/foo/bar",
        d().repr(Https).auth("user:password"),
    );
    check(
        "https://user:password@bitbucket.org/foo/bar#branch",
        d().repr(Https).auth("user:password").committish("branch"),
    );
    check(
        "https://:password@bitbucket.org/foo/bar",
        d().repr(Https).auth(":password"),
    );
    check(
        "https://:password@bitbucket.org/foo/bar#branch",
        d().repr(Https).auth(":password").committish("branch"),
    );

    check("https://bitbucket.org/foo/bar.git", d().repr(Https));
    check(
        "https://bitbucket.org/foo/bar.git#branch",
        d().repr(Https).committish("branch"),
    );
    check(
        "https://user@bitbucket.org/foo/bar.git",
        d().repr(Https).auth("user"),
    );
    check(
        "https://user@bitbucket.org/foo/bar.git#branch",
        d().repr(Https).auth("user").committish("branch"),
    );
    check(
        "https://user:password@bitbucket.org/foo/bar.git",
        d().repr(Https).auth("user:password"),
    );
    check(
        "https://user:password@bitbucket.org/foo/bar.git#branch",
        d().repr(Https).auth("user:password").committish("branch"),
    );
    check(
        "https://:password@bitbucket.org/foo/bar.git",
        d().repr(Https).auth(":password"),
    );
    check(
        "https://:password@bitbucket.org/foo/bar.git#branch",
        d().repr(Https).auth(":password").committish("branch"),
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
