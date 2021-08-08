use claim::*;
use hosted_git_info::DefaultRepresentation::*;
use hosted_git_info::{HostedGitInfo, HostedGitInfoBuilder, Provider};

// default
fn d() -> HostedGitInfoBuilder {
    HostedGitInfoBuilder::default()
        .provider(Provider::Gist)
        .project("feedbeef")
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
    check("gist:feedbeef", d().repr(Shortcut));
    check(
        "gist:feedbeef#branch",
        d().repr(Shortcut).committish("branch"),
    );
    check("gist:user@feedbeef", d().repr(Shortcut));
    check(
        "gist:user@feedbeef#branch",
        d().repr(Shortcut).committish("branch"),
    );
    check("gist:user:password@feedbeef", d().repr(Shortcut));
    check(
        "gist:user:password@feedbeef#branch",
        d().repr(Shortcut).committish("branch"),
    );
    check("gist::password@feedbeef", d().repr(Shortcut));
    check(
        "gist::password@feedbeef#branch",
        d().repr(Shortcut).committish("branch"),
    );

    check("gist:feedbeef.git", d().repr(Shortcut));
    check(
        "gist:feedbeef.git#branch",
        d().repr(Shortcut).committish("branch"),
    );
    check("gist:user@feedbeef.git", d().repr(Shortcut));
    check(
        "gist:user@feedbeef.git#branch",
        d().repr(Shortcut).committish("branch"),
    );
    check("gist:user:password@feedbeef.git", d().repr(Shortcut));
    check(
        "gist:user:password@feedbeef.git#branch",
        d().repr(Shortcut).committish("branch"),
    );
    check("gist::password@feedbeef.git", d().repr(Shortcut));
    check(
        "gist::password@feedbeef.git#branch",
        d().repr(Shortcut).committish("branch"),
    );

    check("gist:/feedbeef", d().repr(Shortcut));
    check(
        "gist:/feedbeef#branch",
        d().repr(Shortcut).committish("branch"),
    );
    check("gist:user@/feedbeef", d().repr(Shortcut));
    check(
        "gist:user@/feedbeef#branch",
        d().repr(Shortcut).committish("branch"),
    );
    check("gist:user:password@/feedbeef", d().repr(Shortcut));
    check(
        "gist:user:password@/feedbeef#branch",
        d().repr(Shortcut).committish("branch"),
    );
    check("gist::password@/feedbeef", d().repr(Shortcut));
    check(
        "gist::password@/feedbeef#branch",
        d().repr(Shortcut).committish("branch"),
    );

    check("gist:/feedbeef.git", d().repr(Shortcut));
    check(
        "gist:/feedbeef.git#branch",
        d().repr(Shortcut).committish("branch"),
    );
    check("gist:user@/feedbeef.git", d().repr(Shortcut));
    check(
        "gist:user@/feedbeef.git#branch",
        d().repr(Shortcut).committish("branch"),
    );
    check("gist:user:password@/feedbeef.git", d().repr(Shortcut));
    check(
        "gist:user:password@/feedbeef.git#branch",
        d().repr(Shortcut).committish("branch"),
    );
    check("gist::password@/feedbeef.git", d().repr(Shortcut));
    check(
        "gist::password@/feedbeef.git#branch",
        d().repr(Shortcut).committish("branch"),
    );

    check("gist:foo/feedbeef", d().repr(Shortcut).user("foo"));
    check(
        "gist:foo/feedbeef#branch",
        d().repr(Shortcut).user("foo").committish("branch"),
    );
    check("gist:user@foo/feedbeef", d().repr(Shortcut).user("foo"));
    check(
        "gist:user@foo/feedbeef#branch",
        d().repr(Shortcut).user("foo").committish("branch"),
    );
    check(
        "gist:user:password@foo/feedbeef",
        d().repr(Shortcut).user("foo"),
    );
    check(
        "gist:user:password@foo/feedbeef#branch",
        d().repr(Shortcut).user("foo").committish("branch"),
    );
    check(
        "gist::password@foo/feedbeef",
        d().repr(Shortcut).user("foo"),
    );
    check(
        "gist::password@foo/feedbeef#branch",
        d().repr(Shortcut).user("foo").committish("branch"),
    );

    check("gist:foo/feedbeef.git", d().repr(Shortcut).user("foo"));
    check(
        "gist:foo/feedbeef.git#branch",
        d().repr(Shortcut).user("foo").committish("branch"),
    );
    check("gist:user@foo/feedbeef.git", d().repr(Shortcut).user("foo"));
    check(
        "gist:user@foo/feedbeef.git#branch",
        d().repr(Shortcut).user("foo").committish("branch"),
    );
    check(
        "gist:user:password@foo/feedbeef.git",
        d().repr(Shortcut).user("foo"),
    );
    check(
        "gist:user:password@foo/feedbeef.git#branch",
        d().repr(Shortcut).user("foo").committish("branch"),
    );
    check(
        "gist::password@foo/feedbeef.git",
        d().repr(Shortcut).user("foo"),
    );
    check(
        "gist::password@foo/feedbeef.git#branch",
        d().repr(Shortcut).user("foo").committish("branch"),
    );
}

#[test]
fn git_urls() {
    // NOTE auth is accepted and respected
    check("git://gist.github.com/feedbeef", d().repr(Git));
    check(
        "git://gist.github.com/feedbeef#branch",
        d().repr(Git).committish("branch"),
    );
    check(
        "git://user@gist.github.com/feedbeef",
        d().repr(Git).auth("user"),
    );
    check(
        "git://user@gist.github.com/feedbeef#branch",
        d().repr(Git).auth("user").committish("branch"),
    );
    check(
        "git://user:password@gist.github.com/feedbeef",
        d().repr(Git).auth("user:password"),
    );
    check(
        "git://user:password@gist.github.com/feedbeef#branch",
        d().repr(Git).auth("user:password").committish("branch"),
    );
    check(
        "git://:password@gist.github.com/feedbeef",
        d().repr(Git).auth(":password"),
    );
    check(
        "git://:password@gist.github.com/feedbeef#branch",
        d().repr(Git).auth(":password").committish("branch"),
    );

    check("git://gist.github.com/feedbeef.git", d().repr(Git));
    check(
        "git://gist.github.com/feedbeef.git#branch",
        d().repr(Git).committish("branch"),
    );
    check(
        "git://user@gist.github.com/feedbeef.git",
        d().repr(Git).auth("user"),
    );
    check(
        "git://user@gist.github.com/feedbeef.git#branch",
        d().repr(Git).auth("user").committish("branch"),
    );
    check(
        "git://user:password@gist.github.com/feedbeef.git",
        d().repr(Git).auth("user:password"),
    );
    check(
        "git://user:password@gist.github.com/feedbeef.git#branch",
        d().repr(Git).auth("user:password").committish("branch"),
    );
    check(
        "git://:password@gist.github.com/feedbeef.git",
        d().repr(Git).auth(":password"),
    );
    check(
        "git://:password@gist.github.com/feedbeef.git#branch",
        d().repr(Git).auth(":password").committish("branch"),
    );

    check(
        "git://gist.github.com/foo/feedbeef",
        d().repr(Git).user("foo"),
    );
    check(
        "git://gist.github.com/foo/feedbeef#branch",
        d().repr(Git).user("foo").committish("branch"),
    );
    check(
        "git://user@gist.github.com/foo/feedbeef",
        d().repr(Git).user("foo").auth("user"),
    );
    check(
        "git://user@gist.github.com/foo/feedbeef#branch",
        d().repr(Git).user("foo").auth("user").committish("branch"),
    );
    check(
        "git://user:password@gist.github.com/foo/feedbeef",
        d().repr(Git).user("foo").auth("user:password"),
    );
    check(
        "git://user:password@gist.github.com/foo/feedbeef#branch",
        d().repr(Git)
            .user("foo")
            .auth("user:password")
            .committish("branch"),
    );
    check(
        "git://:password@gist.github.com/foo/feedbeef",
        d().repr(Git).user("foo").auth(":password"),
    );
    check(
        "git://:password@gist.github.com/foo/feedbeef#branch",
        d().repr(Git)
            .user("foo")
            .auth(":password")
            .committish("branch"),
    );

    check(
        "git://gist.github.com/foo/feedbeef.git",
        d().repr(Git).user("foo"),
    );
    check(
        "git://gist.github.com/foo/feedbeef.git#branch",
        d().repr(Git).user("foo").committish("branch"),
    );
    check(
        "git://user@gist.github.com/foo/feedbeef.git",
        d().repr(Git).user("foo").auth("user"),
    );
    check(
        "git://user@gist.github.com/foo/feedbeef.git#branch",
        d().repr(Git).user("foo").auth("user").committish("branch"),
    );
    check(
        "git://user:password@gist.github.com/foo/feedbeef.git",
        d().repr(Git).user("foo").auth("user:password"),
    );
    check(
        "git://user:password@gist.github.com/foo/feedbeef.git#branch",
        d().repr(Git)
            .user("foo")
            .auth("user:password")
            .committish("branch"),
    );
    check(
        "git://:password@gist.github.com/foo/feedbeef.git",
        d().repr(Git).user("foo").auth(":password"),
    );
    check(
        "git://:password@gist.github.com/foo/feedbeef.git#branch",
        d().repr(Git)
            .user("foo")
            .auth(":password")
            .committish("branch"),
    );
}

#[test]
fn no_protocol() {
    // NOTE auth is accepted and ignored
    check("git@gist.github.com:feedbeef", d().repr(Ssh));
    check(
        "git@gist.github.com:feedbeef#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("user@gist.github.com:feedbeef", d().repr(Ssh));
    check(
        "user@gist.github.com:feedbeef#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("user:password@gist.github.com:feedbeef", d().repr(Ssh));
    check(
        "user:password@gist.github.com:feedbeef#branch",
        d().repr(Ssh).committish("branch"),
    );
    check(":password@gist.github.com:feedbeef", d().repr(Ssh));
    check(
        ":password@gist.github.com:feedbeef#branch",
        d().repr(Ssh).committish("branch"),
    );

    check("git@gist.github.com:feedbeef.git", d().repr(Ssh));
    check(
        "git@gist.github.com:feedbeef.git#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("user@gist.github.com:feedbeef.git", d().repr(Ssh));
    check(
        "user@gist.github.com:feedbeef.git#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("user:password@gist.github.com:feedbeef.git", d().repr(Ssh));
    check(
        "user:password@gist.github.com:feedbeef.git#branch",
        d().repr(Ssh).committish("branch"),
    );
    check(":password@gist.github.com:feedbeef.git", d().repr(Ssh));
    check(
        ":password@gist.github.com:feedbeef.git#branch",
        d().repr(Ssh).committish("branch"),
    );

    check(
        "git@gist.github.com:foo/feedbeef",
        d().repr(Ssh).user("foo"),
    );
    check(
        "git@gist.github.com:foo/feedbeef#branch",
        d().repr(Ssh).user("foo").committish("branch"),
    );
    check(
        "user@gist.github.com:foo/feedbeef",
        d().repr(Ssh).user("foo"),
    );
    check(
        "user@gist.github.com:foo/feedbeef#branch",
        d().repr(Ssh).user("foo").committish("branch"),
    );
    check(
        "user:password@gist.github.com:foo/feedbeef",
        d().repr(Ssh).user("foo"),
    );
    check(
        "user:password@gist.github.com:foo/feedbeef#branch",
        d().repr(Ssh).user("foo").committish("branch"),
    );
    check(
        ":password@gist.github.com:foo/feedbeef",
        d().repr(Ssh).user("foo"),
    );
    check(
        ":password@gist.github.com:foo/feedbeef#branch",
        d().repr(Ssh).user("foo").committish("branch"),
    );

    check(
        "git@gist.github.com:foo/feedbeef.git",
        d().repr(Ssh).user("foo"),
    );
    check(
        "git@gist.github.com:foo/feedbeef.git#branch",
        d().repr(Ssh).user("foo").committish("branch"),
    );
    check(
        "user@gist.github.com:foo/feedbeef.git",
        d().repr(Ssh).user("foo"),
    );
    check(
        "user@gist.github.com:foo/feedbeef.git#branch",
        d().repr(Ssh).user("foo").committish("branch"),
    );
    check(
        "user:password@gist.github.com:foo/feedbeef.git",
        d().repr(Ssh).user("foo"),
    );
    check(
        "user:password@gist.github.com:foo/feedbeef.git#branch",
        d().repr(Ssh).user("foo").committish("branch"),
    );
    check(
        ":password@gist.github.com:foo/feedbeef.git",
        d().repr(Ssh).user("foo"),
    );
    check(
        ":password@gist.github.com:foo/feedbeef.git#branch",
        d().repr(Ssh).user("foo").committish("branch"),
    );
}

#[test]
fn git_ssh_urls() {
    // NOTE auth is accepted but ignored
    // NOTE see TODO at list of invalids, some inputs fail and shouldn"t
    check("git+ssh://gist.github.com:feedbeef", d().repr(Ssh));
    check(
        "git+ssh://gist.github.com:feedbeef#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("git+ssh://user@gist.github.com:feedbeef", d().repr(Ssh));
    check(
        "git+ssh://user@gist.github.com:feedbeef#branch",
        d().repr(Ssh).committish("branch"),
    );
    check(
        "git+ssh://user:password@gist.github.com:feedbeef",
        d().repr(Ssh),
    );
    check(
        "git+ssh://user:password@gist.github.com:feedbeef#branch",
        d().repr(Ssh).committish("branch"),
    );
    check(
        "git+ssh://:password@gist.github.com:feedbeef",
        d().repr(Ssh),
    );
    check(
        "git+ssh://:password@gist.github.com:feedbeef#branch",
        d().repr(Ssh).committish("branch"),
    );

    check("git+ssh://gist.github.com:feedbeef.git", d().repr(Ssh));
    check(
        "git+ssh://gist.github.com:feedbeef.git#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("git+ssh://user@gist.github.com:feedbeef.git", d().repr(Ssh));
    check(
        "git+ssh://user@gist.github.com:feedbeef.git#branch",
        d().repr(Ssh).committish("branch"),
    );
    check(
        "git+ssh://user:password@gist.github.com:feedbeef.git",
        d().repr(Ssh),
    );
    check(
        "git+ssh://user:password@gist.github.com:feedbeef.git#branch",
        d().repr(Ssh).committish("branch"),
    );
    check(
        "git+ssh://:password@gist.github.com:feedbeef.git",
        d().repr(Ssh),
    );
    check(
        "git+ssh://:password@gist.github.com:feedbeef.git#branch",
        d().repr(Ssh).committish("branch"),
    );

    check(
        "git+ssh://gist.github.com:foo/feedbeef",
        d().repr(Ssh).user("foo"),
    );
    check(
        "git+ssh://gist.github.com:foo/feedbeef#branch",
        d().repr(Ssh).user("foo").committish("branch"),
    );
    check(
        "git+ssh://user@gist.github.com:foo/feedbeef",
        d().repr(Ssh).user("foo"),
    );
    check(
        "git+ssh://user@gist.github.com:foo/feedbeef#branch",
        d().repr(Ssh).user("foo").committish("branch"),
    );
    check(
        "git+ssh://user:password@gist.github.com:foo/feedbeef",
        d().repr(Ssh).user("foo"),
    );
    check(
        "git+ssh://user:password@gist.github.com:foo/feedbeef#branch",
        d().repr(Ssh).user("foo").committish("branch"),
    );
    check(
        "git+ssh://:password@gist.github.com:foo/feedbeef",
        d().repr(Ssh).user("foo"),
    );
    check(
        "git+ssh://:password@gist.github.com:foo/feedbeef#branch",
        d().repr(Ssh).user("foo").committish("branch"),
    );

    check(
        "git+ssh://gist.github.com:foo/feedbeef.git",
        d().repr(Ssh).user("foo"),
    );
    check(
        "git+ssh://gist.github.com:foo/feedbeef.git#branch",
        d().repr(Ssh).user("foo").committish("branch"),
    );
    check(
        "git+ssh://user@gist.github.com:foo/feedbeef.git",
        d().repr(Ssh).user("foo"),
    );
    check(
        "git+ssh://user@gist.github.com:foo/feedbeef.git#branch",
        d().repr(Ssh).user("foo").committish("branch"),
    );
    check(
        "git+ssh://user:password@gist.github.com:foo/feedbeef.git",
        d().repr(Ssh).user("foo"),
    );
    check(
        "git+ssh://user:password@gist.github.com:foo/feedbeef.git#branch",
        d().repr(Ssh).user("foo").committish("branch"),
    );
    check(
        "git+ssh://:password@gist.github.com:foo/feedbeef.git",
        d().repr(Ssh).user("foo"),
    );
    check(
        "git+ssh://:password@gist.github.com:foo/feedbeef.git#branch",
        d().repr(Ssh).user("foo").committish("branch"),
    );
}

#[test]
fn ssh_urls() {
    // NOTE auth is accepted but ignored
    check("ssh://gist.github.com:feedbeef", d().repr(Ssh));
    check(
        "ssh://gist.github.com:feedbeef#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("ssh://user@gist.github.com:feedbeef", d().repr(Ssh));
    check(
        "ssh://user@gist.github.com:feedbeef#branch",
        d().repr(Ssh).committish("branch"),
    );
    check(
        "ssh://user:password@gist.github.com:feedbeef",
        d().repr(Ssh),
    );
    check(
        "ssh://user:password@gist.github.com:feedbeef#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("ssh://:password@gist.github.com:feedbeef", d().repr(Ssh));
    check(
        "ssh://:password@gist.github.com:feedbeef#branch",
        d().repr(Ssh).committish("branch"),
    );

    check("ssh://gist.github.com:feedbeef.git", d().repr(Ssh));
    check(
        "ssh://gist.github.com:feedbeef.git#branch",
        d().repr(Ssh).committish("branch"),
    );
    check("ssh://user@gist.github.com:feedbeef.git", d().repr(Ssh));
    check(
        "ssh://user@gist.github.com:feedbeef.git#branch",
        d().repr(Ssh).committish("branch"),
    );
    check(
        "ssh://user:password@gist.github.com:feedbeef.git",
        d().repr(Ssh),
    );
    check(
        "ssh://user:password@gist.github.com:feedbeef.git#branch",
        d().repr(Ssh).committish("branch"),
    );
    check(
        "ssh://:password@gist.github.com:feedbeef.git",
        d().repr(Ssh),
    );
    check(
        "ssh://:password@gist.github.com:feedbeef.git#branch",
        d().repr(Ssh).committish("branch"),
    );

    check(
        "ssh://gist.github.com:foo/feedbeef",
        d().repr(Ssh).user("foo"),
    );
    check(
        "ssh://gist.github.com:foo/feedbeef#branch",
        d().repr(Ssh).user("foo").committish("branch"),
    );
    check(
        "ssh://user@gist.github.com:foo/feedbeef",
        d().repr(Ssh).user("foo"),
    );
    check(
        "ssh://user@gist.github.com:foo/feedbeef#branch",
        d().repr(Ssh).user("foo").committish("branch"),
    );
    check(
        "ssh://user:password@gist.github.com:foo/feedbeef",
        d().repr(Ssh).user("foo"),
    );
    check(
        "ssh://user:password@gist.github.com:foo/feedbeef#branch",
        d().repr(Ssh).user("foo").committish("branch"),
    );
    check(
        "ssh://:password@gist.github.com:foo/feedbeef",
        d().repr(Ssh).user("foo"),
    );
    check(
        "ssh://:password@gist.github.com:foo/feedbeef#branch",
        d().repr(Ssh).user("foo").committish("branch"),
    );

    check(
        "ssh://gist.github.com:foo/feedbeef.git",
        d().repr(Ssh).user("foo"),
    );
    check(
        "ssh://gist.github.com:foo/feedbeef.git#branch",
        d().repr(Ssh).user("foo").committish("branch"),
    );
    check(
        "ssh://user@gist.github.com:foo/feedbeef.git",
        d().repr(Ssh).user("foo"),
    );
    check(
        "ssh://user@gist.github.com:foo/feedbeef.git#branch",
        d().repr(Ssh).user("foo").committish("branch"),
    );
    check(
        "ssh://user:password@gist.github.com:foo/feedbeef.git",
        d().repr(Ssh).user("foo"),
    );
    check(
        "ssh://user:password@gist.github.com:foo/feedbeef.git#branch",
        d().repr(Ssh).user("foo").committish("branch"),
    );
    check(
        "ssh://:password@gist.github.com:foo/feedbeef.git",
        d().repr(Ssh).user("foo"),
    );
    check(
        "ssh://:password@gist.github.com:foo/feedbeef.git#branch",
        d().repr(Ssh).user("foo").committish("branch"),
    );
}

#[test]
fn git_https_urls() {
    // NOTE auth is accepted and respected
    check("git+https://gist.github.com/feedbeef", d().repr(Https));
    check(
        "git+https://gist.github.com/feedbeef#branch",
        d().repr(Https).committish("branch"),
    );
    check(
        "git+https://user@gist.github.com/feedbeef",
        d().repr(Https).auth("user"),
    );
    check(
        "git+https://user@gist.github.com/feedbeef#branch",
        d().repr(Https).auth("user").committish("branch"),
    );
    check(
        "git+https://user:password@gist.github.com/feedbeef",
        d().repr(Https).auth("user:password"),
    );
    check(
        "git+https://user:password@gist.github.com/feedbeef#branch",
        d().repr(Https).auth("user:password").committish("branch"),
    );
    check(
        "git+https://:password@gist.github.com/feedbeef",
        d().repr(Https).auth(":password"),
    );
    check(
        "git+https://:password@gist.github.com/feedbeef#branch",
        d().repr(Https).auth(":password").committish("branch"),
    );

    check("git+https://gist.github.com/feedbeef.git", d().repr(Https));
    check(
        "git+https://gist.github.com/feedbeef.git#branch",
        d().repr(Https).committish("branch"),
    );
    check(
        "git+https://user@gist.github.com/feedbeef.git",
        d().repr(Https).auth("user"),
    );
    check(
        "git+https://user@gist.github.com/feedbeef.git#branch",
        d().repr(Https).auth("user").committish("branch"),
    );
    check(
        "git+https://user:password@gist.github.com/feedbeef.git",
        d().repr(Https).auth("user:password"),
    );
    check(
        "git+https://user:password@gist.github.com/feedbeef.git#branch",
        d().repr(Https).auth("user:password").committish("branch"),
    );
    check(
        "git+https://:password@gist.github.com/feedbeef.git",
        d().repr(Https).auth(":password"),
    );
    check(
        "git+https://:password@gist.github.com/feedbeef.git#branch",
        d().repr(Https).auth(":password").committish("branch"),
    );

    check(
        "git+https://gist.github.com/foo/feedbeef",
        d().repr(Https).user("foo"),
    );
    check(
        "git+https://gist.github.com/foo/feedbeef#branch",
        d().repr(Https).user("foo").committish("branch"),
    );
    check(
        "git+https://user@gist.github.com/foo/feedbeef",
        d().repr(Https).auth("user").user("foo"),
    );
    check(
        "git+https://user@gist.github.com/foo/feedbeef#branch",
        d().repr(Https)
            .auth("user")
            .user("foo")
            .committish("branch"),
    );
    check(
        "git+https://user:password@gist.github.com/foo/feedbeef",
        d().repr(Https).auth("user:password").user("foo"),
    );
    check(
        "git+https://user:password@gist.github.com/foo/feedbeef#branch",
        d().repr(Https)
            .auth("user:password")
            .user("foo")
            .committish("branch"),
    );
    check(
        "git+https://:password@gist.github.com/foo/feedbeef",
        d().repr(Https).auth(":password").user("foo"),
    );
    check(
        "git+https://:password@gist.github.com/foo/feedbeef#branch",
        d().repr(Https)
            .auth(":password")
            .user("foo")
            .committish("branch"),
    );

    check(
        "git+https://gist.github.com/foo/feedbeef.git",
        d().repr(Https).user("foo"),
    );
    check(
        "git+https://gist.github.com/foo/feedbeef.git#branch",
        d().repr(Https).user("foo").committish("branch"),
    );
    check(
        "git+https://user@gist.github.com/foo/feedbeef.git",
        d().repr(Https).auth("user").user("foo"),
    );
    check(
        "git+https://user@gist.github.com/foo/feedbeef.git#branch",
        d().repr(Https)
            .auth("user")
            .user("foo")
            .committish("branch"),
    );
    check(
        "git+https://user:password@gist.github.com/foo/feedbeef.git",
        d().repr(Https).auth("user:password").user("foo"),
    );
    check(
        "git+https://user:password@gist.github.com/foo/feedbeef.git#branch",
        d().repr(Https)
            .auth("user:password")
            .user("foo")
            .committish("branch"),
    );
    check(
        "git+https://:password@gist.github.com/foo/feedbeef.git",
        d().repr(Https).auth(":password").user("foo"),
    );
    check(
        "git+https://:password@gist.github.com/foo/feedbeef.git#branch",
        d().repr(Https)
            .auth(":password")
            .user("foo")
            .committish("branch"),
    );
}

#[test]
fn https_urls() {
    // NOTE auth is accepted and respected
    check("https://gist.github.com/feedbeef", d().repr(Https));
    check(
        "https://gist.github.com/feedbeef#branch",
        d().repr(Https).committish("branch"),
    );
    check(
        "https://user@gist.github.com/feedbeef",
        d().repr(Https).auth("user"),
    );
    check(
        "https://user@gist.github.com/feedbeef#branch",
        d().repr(Https).auth("user").committish("branch"),
    );
    check(
        "https://user:password@gist.github.com/feedbeef",
        d().repr(Https).auth("user:password"),
    );
    check(
        "https://user:password@gist.github.com/feedbeef#branch",
        d().repr(Https).auth("user:password").committish("branch"),
    );
    check(
        "https://:password@gist.github.com/feedbeef",
        d().repr(Https).auth(":password"),
    );
    check(
        "https://:password@gist.github.com/feedbeef#branch",
        d().repr(Https).auth(":password").committish("branch"),
    );

    check("https://gist.github.com/feedbeef.git", d().repr(Https));
    check(
        "https://gist.github.com/feedbeef.git#branch",
        d().repr(Https).committish("branch"),
    );
    check(
        "https://user@gist.github.com/feedbeef.git",
        d().repr(Https).auth("user"),
    );
    check(
        "https://user@gist.github.com/feedbeef.git#branch",
        d().repr(Https).auth("user").committish("branch"),
    );
    check(
        "https://user:password@gist.github.com/feedbeef.git",
        d().repr(Https).auth("user:password"),
    );
    check(
        "https://user:password@gist.github.com/feedbeef.git#branch",
        d().repr(Https).auth("user:password").committish("branch"),
    );
    check(
        "https://:password@gist.github.com/feedbeef.git",
        d().repr(Https).auth(":password"),
    );
    check(
        "https://:password@gist.github.com/feedbeef.git#branch",
        d().repr(Https).auth(":password").committish("branch"),
    );

    check(
        "https://gist.github.com/foo/feedbeef",
        d().repr(Https).user("foo"),
    );
    check(
        "https://gist.github.com/foo/feedbeef#branch",
        d().repr(Https).user("foo").committish("branch"),
    );
    check(
        "https://user@gist.github.com/foo/feedbeef",
        d().repr(Https).auth("user").user("foo"),
    );
    check(
        "https://user@gist.github.com/foo/feedbeef#branch",
        d().repr(Https)
            .auth("user")
            .user("foo")
            .committish("branch"),
    );
    check(
        "https://user:password@gist.github.com/foo/feedbeef",
        d().repr(Https).auth("user:password").user("foo"),
    );
    check(
        "https://user:password@gist.github.com/foo/feedbeef#branch",
        d().repr(Https)
            .auth("user:password")
            .user("foo")
            .committish("branch"),
    );
    check(
        "https://:password@gist.github.com/foo/feedbeef",
        d().repr(Https).auth(":password").user("foo"),
    );
    check(
        "https://:password@gist.github.com/foo/feedbeef#branch",
        d().repr(Https)
            .auth(":password")
            .user("foo")
            .committish("branch"),
    );

    check(
        "https://gist.github.com/foo/feedbeef.git",
        d().repr(Https).user("foo"),
    );
    check(
        "https://gist.github.com/foo/feedbeef.git#branch",
        d().repr(Https).user("foo").committish("branch"),
    );
    check(
        "https://user@gist.github.com/foo/feedbeef.git",
        d().repr(Https).auth("user").user("foo"),
    );
    check(
        "https://user@gist.github.com/foo/feedbeef.git#branch",
        d().repr(Https)
            .auth("user")
            .user("foo")
            .committish("branch"),
    );
    check(
        "https://user:password@gist.github.com/foo/feedbeef.git",
        d().repr(Https).auth("user:password").user("foo"),
    );
    check(
        "https://user:password@gist.github.com/foo/feedbeef.git#branch",
        d().repr(Https)
            .auth("user:password")
            .user("foo")
            .committish("branch"),
    );
    check(
        "https://:password@gist.github.com/foo/feedbeef.git",
        d().repr(Https).auth(":password").user("foo"),
    );
    check(
        "https://:password@gist.github.com/foo/feedbeef.git#branch",
        d().repr(Https)
            .auth(":password")
            .user("foo")
            .committish("branch"),
    );
}

#[test]
fn invalid() {
    // raw urls that are wrong anyway but for some reason are in the wild
    check_err("https://gist.github.com/foo/feedbeef/raw/fix%2Fbug/");
    // missing both user and project
    check_err("https://gist.github.com/");
}
