hosted-git-info
==============================================================================

Provides metadata and conversions from repository urls for [GitHub], [Bitbucket]
and [GitLab].

[GitHub]: https://github.com/
[Bitbucket]: https://www.bitbucket.org/
[GitLab]: https://www.gitlab.com/

__This is a [Rust] port of the original [hosted-git-info] project on [npm].__

[Rust]: https://www.rustlang.org/
[hosted-git-info]: https://github.com/npm/hosted-git-info
[npm]: https://www.npmjs.com

> This will let you identify and transform various git hosts URLs between
> protocols.  It also can tell you what the URL is for the raw path for
> particular file for direct access without git.


Usage
------------------------------------------------------------------------------

```rust
use hosted_git_info::HostedGitInfo;

fn main() {
    let url = "https://github.com/foo/bar.git#branch";
    let info = HostedGitInfo::from_url(url).unwrap();
    assert_eq!(info.provider, Provider::GitHub);
    assert_eq!(info.user, Some("foo"));
    assert_eq!(info.project, "bar");
    assert_eq!(info.committish, Some("branch"));
}
```


Related
------------------------------------------------------------------------------

- [hosted-git-info] – The original library for JavaScript


License
------------------------------------------------------------------------------

This project is licensed under the ISC license ([LICENSE](LICENSE) or
<http://opensource.org/licenses/ISC>).
