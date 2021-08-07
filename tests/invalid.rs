use claim::*;
use hosted_git_info::HostedGitInfo;

#[track_caller]
fn check_err(input: &str) {
    let result = HostedGitInfo::from_url(input);
    assert_err!(result, "{} does not match expectation", input);
}

#[test]
fn invalid() {
    check_err("https://google.com");
    check_err("git+ssh://git@nothosted.com/abc/def");
    check_err("git://nothosted.com");
    check_err("git+file:///foo/bar");
    check_err("git+ssh://git@git.unlucky.com:RND/electron-tools/some-tool#2.0.1");
    check_err("::");
    check_err("");
}
