use crate::error;

pub struct GitHub {
    user: String,
    repo: String,
}

impl GitHub {
    pub fn new(user: impl Into<String>, repo: impl Into<String>) -> Self {
        Self {
            user: user.into(),
            repo: repo.into(),
        }
    }

    pub fn call(args: impl IntoIterator<Item = impl Into<String>>) -> Result<String, String> {
        let args: Vec<String> = args.into_iter().map(|e| e.into()).collect();
        let output = std::process::Command::new("gh")
            .args(&args)
            .output()
            .map_err(|err| format!("failed to exec gh {args:?}: {err:?}"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("gh returned an error:\n{}", stderr.trim()));
        }

        let stdout = String::from_utf8(output.stdout)
            .map_err(|err| format!("non-utf8 output from gh: {err:?}"))?;

        Ok(stdout.trim().to_string())
    }

    pub fn latest_release(&self) -> Result<String, String> {
        Self::call([
            "release",
            "view",
            "-R",
            &format!("{}/{}", self.user, self.repo),
            "--json",
            "tagName",
            "--jq",
            ".tagName",
        ])
    }

    pub fn latest_tag(&self) -> Result<String, String> {
        let stdout = Self::call([
            "api",
            "-H",
            "Accept: application/vnd.github+json",
            "-H",
            "X-GitHub-Api-Version: 2022-11-28",
            &format!("/repos/{}/{}/tags", self.user, self.repo),
            "--jq",
            ".[] | .name ",
        ])?;

        stdout
            .lines()
            .max()
            .map(|v| v.to_string())
            .ok_or_else(|| "no tags".to_string())
    }

    pub fn list_assets(&self, release_name: impl AsRef<str>) -> Result<Vec<GitHubAsset>, String> {
        let release_name = release_name.as_ref();
        const PREFIX: &str = "asset:\t";
        let stdout = Self::call(["release", "view", release_name])?;
        let assets = stdout
            .lines()
            .filter_map(|line| line.strip_prefix(PREFIX))
            .map(GitHubAsset::new)
            .collect::<Vec<_>>();
        Ok(assets)
    }

    pub fn delete_asset(
        &self,
        release_name: impl AsRef<str>,
        asset: &GitHubAsset,
    ) -> Result<String, String> {
        Self::call([
            "release",
            "delete-asset",
            release_name.as_ref(),
            asset.filename.as_str(),
            "--yes",
        ])
    }

    pub fn upload_asset(
        &self,
        release_name: impl AsRef<str>,
        asset: &GitHubAsset,
    ) -> Result<String, String> {
        Self::call([
            "release",
            "upload",
            release_name.as_ref(),
            asset.filename.as_str(),
        ])
    }
}

#[derive(Clone)]
pub struct GitHubAsset {
    pub filename: String,
    pub package_name: String,
}

impl GitHubAsset {
    pub fn new(filename: impl Into<String>) -> Self {
        let filename = filename.into();
        let package_name = filename
            .split_once('_')
            .unwrap_or_else(|| {
                const HINT: &str = "must have a format <name>_<version>_<arch>.deb";
                error!("filename {filename:?} is not a valid debian package filename; {HINT}")
            })
            .0
            .to_string();

        Self {
            filename,
            package_name,
        }
    }
}

impl std::fmt::Display for GitHubAsset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.filename, self.package_name)
    }
}
