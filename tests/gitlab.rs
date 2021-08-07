use claim::*;
use hosted_git_info::{HostedGitInfo, HostedGitInfoBuilder, Provider};

// default
fn d() -> HostedGitInfoBuilder {
    HostedGitInfoBuilder::default()
        .provider(Provider::GitLab)
        .user("foo")
        .project("bar")
        .clone()
}

// subgroup
fn s() -> HostedGitInfoBuilder {
    d().user("foo/bar").project("baz").clone()
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
    // NOTE subgroups are respected, but the subgroup is treated as the project and the real project is lost
    check("gitlab:foo/bar", &d());
    check("gitlab:foo/bar#branch", d().committish("branch"));
    check("gitlab:user@foo/bar", &d());
    check("gitlab:user@foo/bar#branch", d().committish("branch"));
    check("gitlab:user:password@foo/bar", &d());
    check(
        "gitlab:user:password@foo/bar#branch",
        d().committish("branch"),
    );
    check("gitlab::password@foo/bar", &d());
    check("gitlab::password@foo/bar#branch", d().committish("branch"));

    check("gitlab:foo/bar.git", &d());
    check("gitlab:foo/bar.git#branch", d().committish("branch"));
    check("gitlab:user@foo/bar.git", &d());
    check("gitlab:user@foo/bar.git#branch", d().committish("branch"));
    check("gitlab:user:password@foo/bar.git", &d());
    check(
        "gitlab:user:password@foo/bar.git#branch",
        d().committish("branch"),
    );
    check("gitlab::password@foo/bar.git", &d());
    check(
        "gitlab::password@foo/bar.git#branch",
        d().committish("branch"),
    );

    check("gitlab:foo/bar/baz", &s());
    check("gitlab:foo/bar/baz#branch", s().committish("branch"));
    check("gitlab:user@foo/bar/baz", &s());
    check("gitlab:user@foo/bar/baz#branch", s().committish("branch"));
    check("gitlab:user:password@foo/bar/baz", &s());
    check(
        "gitlab:user:password@foo/bar/baz#branch",
        s().committish("branch"),
    );
    check("gitlab::password@foo/bar/baz", &s());
    check(
        "gitlab::password@foo/bar/baz#branch",
        s().committish("branch"),
    );

    check("gitlab:foo/bar/baz.git", &s());
    check("gitlab:foo/bar/baz.git#branch", s().committish("branch"));
    check("gitlab:user@foo/bar/baz.git", &s());
    check(
        "gitlab:user@foo/bar/baz.git#branch",
        s().committish("branch"),
    );
    check("gitlab:user:password@foo/bar/baz.git", &s());
    check(
        "gitlab:user:password@foo/bar/baz.git#branch",
        s().committish("branch"),
    );
    check("gitlab::password@foo/bar/baz.git", &s());
    check(
        "gitlab::password@foo/bar/baz.git#branch",
        s().committish("branch"),
    );
}

#[test]
fn no_protocol() {
    // NOTE auth is _required_ (see invalid list) but ignored
    check("user@gitlab.com:foo/bar", &d());
    check("user@gitlab.com:foo/bar#branch", d().committish("branch"));
    check("user:password@gitlab.com:foo/bar", &d());
    check(
        "user:password@gitlab.com:foo/bar#branch",
        d().committish("branch"),
    );
    check(":password@gitlab.com:foo/bar", &d());
    check(
        ":password@gitlab.com:foo/bar#branch",
        d().committish("branch"),
    );

    check("user@gitlab.com:foo/bar.git", &d());
    check(
        "user@gitlab.com:foo/bar.git#branch",
        d().committish("branch"),
    );
    check("user:password@gitlab.com:foo/bar.git", &d());
    check(
        "user:password@gitlab.com:foo/bar.git#branch",
        d().committish("branch"),
    );
    check(":password@gitlab.com:foo/bar.git", &d());
    check(
        ":password@gitlab.com:foo/bar.git#branch",
        d().committish("branch"),
    );

    check("user@gitlab.com:foo/bar/baz", &s());
    check(
        "user@gitlab.com:foo/bar/baz#branch",
        s().committish("branch"),
    );
    check("user:password@gitlab.com:foo/bar/baz", &s());
    check(
        "user:password@gitlab.com:foo/bar/baz#branch",
        s().committish("branch"),
    );
    check(":password@gitlab.com:foo/bar/baz", &s());
    check(
        ":password@gitlab.com:foo/bar/baz#branch",
        s().committish("branch"),
    );

    check("user@gitlab.com:foo/bar/baz.git", &s());
    check(
        "user@gitlab.com:foo/bar/baz.git#branch",
        s().committish("branch"),
    );
    check("user:password@gitlab.com:foo/bar/baz.git", &s());
    check(
        "user:password@gitlab.com:foo/bar/baz.git#branch",
        s().committish("branch"),
    );
    check(":password@gitlab.com:foo/bar/baz.git", &s());
    check(
        ":password@gitlab.com:foo/bar/baz.git#branch",
        s().committish("branch"),
    );
}

#[test]
fn git_ssh_urls() {
    // NOTE auth is accepted but ignored
    // NOTE subprojects are accepted, but the subproject is treated as the project and the real project is lost
    check("git+ssh://gitlab.com:foo/bar", &d());
    check(
        "git+ssh://gitlab.com:foo/bar#branch",
        d().committish("branch"),
    );
    check("git+ssh://user@gitlab.com:foo/bar", &d());
    check(
        "git+ssh://user@gitlab.com:foo/bar#branch",
        d().committish("branch"),
    );
    check("git+ssh://user:password@gitlab.com:foo/bar", &d());
    check(
        "git+ssh://user:password@gitlab.com:foo/bar#branch",
        d().committish("branch"),
    );
    check("git+ssh://:password@gitlab.com:foo/bar", &d());
    check(
        "git+ssh://:password@gitlab.com:foo/bar#branch",
        d().committish("branch"),
    );

    check("git+ssh://gitlab.com:foo/bar.git", &d());
    check(
        "git+ssh://gitlab.com:foo/bar.git#branch",
        d().committish("branch"),
    );
    check("git+ssh://user@gitlab.com:foo/bar.git", &d());
    check(
        "git+ssh://user@gitlab.com:foo/bar.git#branch",
        d().committish("branch"),
    );
    check("git+ssh://user:password@gitlab.com:foo/bar.git", &d());
    check(
        "git+ssh://user:password@gitlab.com:foo/bar.git#branch",
        d().committish("branch"),
    );
    check("git+ssh://:password@gitlab.com:foo/bar.git", &d());
    check(
        "git+ssh://:password@gitlab.com:foo/bar.git#branch",
        d().committish("branch"),
    );

    check("git+ssh://gitlab.com:foo/bar/baz", &s());
    check(
        "git+ssh://gitlab.com:foo/bar/baz#branch",
        s().committish("branch"),
    );
    check("git+ssh://user@gitlab.com:foo/bar/baz", &s());
    check(
        "git+ssh://user@gitlab.com:foo/bar/baz#branch",
        s().committish("branch"),
    );
    check("git+ssh://user:password@gitlab.com:foo/bar/baz", &s());
    check(
        "git+ssh://user:password@gitlab.com:foo/bar/baz#branch",
        s().committish("branch"),
    );
    check("git+ssh://:password@gitlab.com:foo/bar/baz", &s());
    check(
        "git+ssh://:password@gitlab.com:foo/bar/baz#branch",
        s().committish("branch"),
    );

    check("git+ssh://gitlab.com:foo/bar/baz.git", &s());
    check(
        "git+ssh://gitlab.com:foo/bar/baz.git#branch",
        s().committish("branch"),
    );
    check("git+ssh://user@gitlab.com:foo/bar/baz.git", &s());
    check(
        "git+ssh://user@gitlab.com:foo/bar/baz.git#branch",
        s().committish("branch"),
    );
    check("git+ssh://user:password@gitlab.com:foo/bar/baz.git", &s());
    check(
        "git+ssh://user:password@gitlab.com:foo/bar/baz.git#branch",
        s().committish("branch"),
    );
    check("git+ssh://:password@gitlab.com:foo/bar/baz.git", &s());
    check(
        "git+ssh://:password@gitlab.com:foo/bar/baz.git#branch",
        s().committish("branch"),
    );
}

#[test]
fn ssh_urls() {
    // NOTE auth is accepted but ignored
    // NOTE subprojects are accepted, but the subproject is treated as the project and the real project is lost
    check("ssh://gitlab.com:foo/bar", &d());
    check("ssh://gitlab.com:foo/bar#branch", d().committish("branch"));
    check("ssh://user@gitlab.com:foo/bar", &d());
    check(
        "ssh://user@gitlab.com:foo/bar#branch",
        d().committish("branch"),
    );
    check("ssh://user:password@gitlab.com:foo/bar", &d());
    check(
        "ssh://user:password@gitlab.com:foo/bar#branch",
        d().committish("branch"),
    );
    check("ssh://:password@gitlab.com:foo/bar", &d());
    check(
        "ssh://:password@gitlab.com:foo/bar#branch",
        d().committish("branch"),
    );

    check("ssh://gitlab.com:foo/bar.git", &d());
    check(
        "ssh://gitlab.com:foo/bar.git#branch",
        d().committish("branch"),
    );
    check("ssh://user@gitlab.com:foo/bar.git", &d());
    check(
        "ssh://user@gitlab.com:foo/bar.git#branch",
        d().committish("branch"),
    );
    check("ssh://user:password@gitlab.com:foo/bar.git", &d());
    check(
        "ssh://user:password@gitlab.com:foo/bar.git#branch",
        d().committish("branch"),
    );
    check("ssh://:password@gitlab.com:foo/bar.git", &d());
    check(
        "ssh://:password@gitlab.com:foo/bar.git#branch",
        d().committish("branch"),
    );

    check("ssh://gitlab.com:foo/bar/baz", &s());
    check(
        "ssh://gitlab.com:foo/bar/baz#branch",
        s().committish("branch"),
    );
    check("ssh://user@gitlab.com:foo/bar/baz", &s());
    check(
        "ssh://user@gitlab.com:foo/bar/baz#branch",
        s().committish("branch"),
    );
    check("ssh://user:password@gitlab.com:foo/bar/baz", &s());
    check(
        "ssh://user:password@gitlab.com:foo/bar/baz#branch",
        s().committish("branch"),
    );
    check("ssh://:password@gitlab.com:foo/bar/baz", &s());
    check(
        "ssh://:password@gitlab.com:foo/bar/baz#branch",
        s().committish("branch"),
    );

    check("ssh://gitlab.com:foo/bar/baz.git", &s());
    check(
        "ssh://gitlab.com:foo/bar/baz.git#branch",
        s().committish("branch"),
    );
    check("ssh://user@gitlab.com:foo/bar/baz.git", &s());
    check(
        "ssh://user@gitlab.com:foo/bar/baz.git#branch",
        s().committish("branch"),
    );
    check("ssh://user:password@gitlab.com:foo/bar/baz.git", &s());
    check(
        "ssh://user:password@gitlab.com:foo/bar/baz.git#branch",
        s().committish("branch"),
    );
    check("ssh://:password@gitlab.com:foo/bar/baz.git", &s());
    check(
        "ssh://:password@gitlab.com:foo/bar/baz.git#branch",
        s().committish("branch"),
    );
}

#[test]
fn git_https_urls() {
    // NOTE auth is accepted and respected
    // NOTE subprojects are accepted, but the subproject is treated as the project and the real project is lost
    check("git+https://gitlab.com/foo/bar", &d());
    check(
        "git+https://gitlab.com/foo/bar#branch",
        d().committish("branch"),
    );
    check("git+https://user@gitlab.com/foo/bar", d().auth("user"));
    check(
        "git+https://user@gitlab.com/foo/bar#branch",
        d().auth("user").committish("branch"),
    );
    check(
        "git+https://user:password@gitlab.com/foo/bar",
        d().auth("user:password"),
    );
    check(
        "git+https://user:password@gitlab.com/foo/bar#branch",
        d().auth("user:password").committish("branch"),
    );
    check(
        "git+https://:password@gitlab.com/foo/bar",
        d().auth(":password"),
    );
    check(
        "git+https://:password@gitlab.com/foo/bar#branch",
        d().auth(":password").committish("branch"),
    );

    check("git+https://gitlab.com/foo/bar.git", &d());
    check(
        "git+https://gitlab.com/foo/bar.git#branch",
        d().committish("branch"),
    );
    check("git+https://user@gitlab.com/foo/bar.git", d().auth("user"));
    check(
        "git+https://user@gitlab.com/foo/bar.git#branch",
        d().auth("user").committish("branch"),
    );
    check(
        "git+https://user:password@gitlab.com/foo/bar.git",
        d().auth("user:password"),
    );
    check(
        "git+https://user:password@gitlab.com/foo/bar.git#branch",
        d().auth("user:password").committish("branch"),
    );
    check(
        "git+https://:password@gitlab.com/foo/bar.git",
        d().auth(":password"),
    );
    check(
        "git+https://:password@gitlab.com/foo/bar.git#branch",
        d().auth(":password").committish("branch"),
    );

    check("git+https://gitlab.com/foo/bar/baz", &s());
    check(
        "git+https://gitlab.com/foo/bar/baz#branch",
        s().committish("branch"),
    );
    check("git+https://user@gitlab.com/foo/bar/baz", s().auth("user"));
    check(
        "git+https://user@gitlab.com/foo/bar/baz#branch",
        s().auth("user").committish("branch"),
    );
    check(
        "git+https://user:password@gitlab.com/foo/bar/baz",
        s().auth("user:password"),
    );
    check(
        "git+https://user:password@gitlab.com/foo/bar/baz#branch",
        s().auth("user:password").committish("branch"),
    );
    check(
        "git+https://:password@gitlab.com/foo/bar/baz",
        s().auth(":password"),
    );
    check(
        "git+https://:password@gitlab.com/foo/bar/baz#branch",
        s().auth(":password").committish("branch"),
    );

    check("git+https://gitlab.com/foo/bar/baz.git", &s());
    check(
        "git+https://gitlab.com/foo/bar/baz.git#branch",
        s().committish("branch"),
    );
    check(
        "git+https://user@gitlab.com/foo/bar/baz.git",
        s().auth("user"),
    );
    check(
        "git+https://user@gitlab.com/foo/bar/baz.git#branch",
        s().auth("user").committish("branch"),
    );
    check(
        "git+https://user:password@gitlab.com/foo/bar/baz.git",
        s().auth("user:password"),
    );
    check(
        "git+https://user:password@gitlab.com/foo/bar/baz.git#branch",
        s().auth("user:password").committish("branch"),
    );
    check(
        "git+https://:password@gitlab.com/foo/bar/baz.git",
        s().auth(":password"),
    );
    check(
        "git+https://:password@gitlab.com/foo/bar/baz.git#branch",
        s().auth(":password").committish("branch"),
    );
}

#[test]
fn https_urls() {
    // NOTE auth is accepted and respected
    // NOTE subprojects are accepted, but the subproject is treated as the project and the real project is lost
    check("https://gitlab.com/foo/bar", &d());
    check(
        "https://gitlab.com/foo/bar#branch",
        d().committish("branch"),
    );
    check("https://user@gitlab.com/foo/bar", d().auth("user"));
    check(
        "https://user@gitlab.com/foo/bar#branch",
        d().auth("user").committish("branch"),
    );
    check(
        "https://user:password@gitlab.com/foo/bar",
        d().auth("user:password"),
    );
    check(
        "https://user:password@gitlab.com/foo/bar#branch",
        d().auth("user:password").committish("branch"),
    );
    check(
        "https://:password@gitlab.com/foo/bar",
        d().auth(":password"),
    );
    check(
        "https://:password@gitlab.com/foo/bar#branch",
        d().auth(":password").committish("branch"),
    );

    check("https://gitlab.com/foo/bar.git", &d());
    check(
        "https://gitlab.com/foo/bar.git#branch",
        d().committish("branch"),
    );
    check("https://user@gitlab.com/foo/bar.git", d().auth("user"));
    check(
        "https://user@gitlab.com/foo/bar.git#branch",
        d().auth("user").committish("branch"),
    );
    check(
        "https://user:password@gitlab.com/foo/bar.git",
        d().auth("user:password"),
    );
    check(
        "https://user:password@gitlab.com/foo/bar.git#branch",
        d().auth("user:password").committish("branch"),
    );
    check(
        "https://:password@gitlab.com/foo/bar.git",
        d().auth(":password"),
    );
    check(
        "https://:password@gitlab.com/foo/bar.git#branch",
        d().auth(":password").committish("branch"),
    );

    check("https://gitlab.com/foo/bar/baz", &s());
    check(
        "https://gitlab.com/foo/bar/baz#branch",
        s().committish("branch"),
    );
    check("https://user@gitlab.com/foo/bar/baz", s().auth("user"));
    check(
        "https://user@gitlab.com/foo/bar/baz#branch",
        s().auth("user").committish("branch"),
    );
    check(
        "https://user:password@gitlab.com/foo/bar/baz",
        s().auth("user:password"),
    );
    check(
        "https://user:password@gitlab.com/foo/bar/baz#branch",
        s().auth("user:password").committish("branch"),
    );
    check(
        "https://:password@gitlab.com/foo/bar/baz",
        s().auth(":password"),
    );
    check(
        "https://:password@gitlab.com/foo/bar/baz#branch",
        s().auth(":password").committish("branch"),
    );

    check("https://gitlab.com/foo/bar/baz.git", &s());
    check(
        "https://gitlab.com/foo/bar/baz.git#branch",
        s().committish("branch"),
    );
    check("https://user@gitlab.com/foo/bar/baz.git", s().auth("user"));
    check(
        "https://user@gitlab.com/foo/bar/baz.git#branch",
        s().auth("user").committish("branch"),
    );
    check(
        "https://user:password@gitlab.com/foo/bar/baz.git",
        s().auth("user:password"),
    );
    check(
        "https://user:password@gitlab.com/foo/bar/baz.git#branch",
        s().auth("user:password").committish("branch"),
    );
    check(
        "https://:password@gitlab.com/foo/bar/baz.git",
        s().auth(":password"),
    );
    check(
        "https://:password@gitlab.com/foo/bar/baz.git#branch",
        s().auth(":password").committish("branch"),
    );
}

#[test]
fn invalid() {
    // gitlab urls can contain a /-/ segment, make sure we ignore those
    check_err("https://gitlab.com/foo/-/something");
    // missing project
    check_err("https://gitlab.com/foo");
    // tarball, this should not parse so that it can be used for pacote"s remote fetcher
    check_err("https://gitlab.com/foo/bar/repository/archive.tar.gz");
    check_err("https://gitlab.com/foo/bar/repository/archive.tar.gz?ref=49b393e2ded775f2df36ef2ffcb61b0359c194c9");
}
