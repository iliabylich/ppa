#[derive(Debug)]
pub(crate) struct Changelog;

impl Changelog {
    pub(crate) fn render(package_name: &str, version: &str) -> String {
        format!(
            "{package_name} ({version}) unstable; urgency=low

  * Release

 -- John Doe <john@doe.org>  Wed, 22 May 2024 17:54:24 +0000
"
        )
    }
}
