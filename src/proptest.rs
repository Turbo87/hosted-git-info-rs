use crate::HostedGitInfo;
use proptest::prelude::*;

proptest! {
    #[test]
    #[allow(unused_must_use)]
    fn doesnt_crash(s in "\\PC*") {
        HostedGitInfo::from_url(&s);
    }

    #[test]
    #[allow(unused_must_use)]
    fn doesnt_crash_from_random_github_shortcut(s in "github:\\PC*") {
        HostedGitInfo::from_url(&s);
    }

    #[test]
    #[allow(unused_must_use)]
    fn doesnt_crash_from_random_github_url_with_no_protocol(s in "\\PC*@github.com:\\PC*") {
        HostedGitInfo::from_url(&s);
    }

    #[test]
    #[allow(unused_must_use)]
    fn doesnt_crash_from_random_gitlab_shortcut(s in "gitlab:\\PC*") {
        HostedGitInfo::from_url(&s);
    }

    #[test]
    #[allow(unused_must_use)]
    fn doesnt_crash_from_random_gitlab_url_with_no_protocol(s in "\\PC*@gitlab.com:\\PC*") {
        HostedGitInfo::from_url(&s);
    }

    #[test]
    #[allow(unused_must_use)]
    fn doesnt_crash_from_random_bitbucket_shortcut(s in "bitbucket:\\PC*") {
        HostedGitInfo::from_url(&s);
    }

    #[test]
    #[allow(unused_must_use)]
    fn doesnt_crash_from_random_bitbucket_url_with_no_protocol(s in "\\PC*@bitbucket.org:\\PC*") {
        HostedGitInfo::from_url(&s);
    }
}
