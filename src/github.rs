pub struct GitHub {
    owner_slash_repo: String,
}

impl GitHub {
    pub fn new(owner_slash_repo: impl Into<String>) -> Self {
        Self {
            owner_slash_repo: owner_slash_repo.into(),
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
            &self.owner_slash_repo,
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
            &format!("/repos/{}/tags", self.owner_slash_repo),
            "--jq",
            ".[] | .name ",
        ])?;

        stdout
            .lines()
            .max()
            .map(|v| v.to_string())
            .ok_or_else(|| "no tags".to_string())
    }

    pub fn list_assets(&self, release_name: impl AsRef<str>) -> Result<Vec<String>, String> {
        const PREFIX: &str = "asset:  ";
        let stdout = Self::call(["release", "view", release_name.as_ref()])?;
        println!("{stdout}");
        todo!()
    }
}
