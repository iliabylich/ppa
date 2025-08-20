use build_deb_package::{
    args::paths_from_args,
    config::{Config, Version},
    error, green, yellow,
};

fn main() {
    for path in paths_from_args() {
        let config = Config::read(path);
        bump_one(config);
    }
}

fn bump_one(config: Config) {
    let Version::Specific(version) = config.version else {
        yellow!("Skipping {} (0-0-timestamp)", config.package_name);
        return;
    };

    let contents = std::fs::read_to_string(&config.filepath)
        .unwrap_or_else(|err| error!(err = err, "failed to read config at {:?}", config.filepath));

    let (pre, post) = split(&contents, &version)
        .unwrap_or_else(|| error!("failed to split config at {:?}", config.filepath));

    let Some(version) = SemanticVersion::parse(&version) else {
        yellow!("Skipping {} (non-semantic version)", config.package_name);
        return;
    };

    let next_version = version.bump();
    green!("{} {version} -> {next_version}", config.package_name);

    std::fs::write(&config.filepath, format!("{pre}{next_version}{post}")).unwrap_or_else(|err| {
        error!(
            err = err,
            "failed to write updated config for {:?}", config.filepath
        )
    });
}

fn split<'a>(contents: &'a str, version: &str) -> Option<(&'a str, &'a str)> {
    let version_start_idx = contents.find(version)?;
    let version_end_idx = version_start_idx + version.len();

    let pre = &contents[..version_start_idx];
    let post = &contents[version_end_idx..];

    Some((pre, post))
}

#[derive(Debug, Clone, Copy)]
struct SemanticVersion {
    major: usize,
    minor: usize,
    patch: usize,
    build: usize,
}

impl SemanticVersion {
    fn parse(text: &str) -> Option<Self> {
        let (major, rest) = text.split_once('.')?;
        let (minor, rest) = rest.split_once('.')?;

        let (patch, build) = if digits_only(rest) {
            (rest, "0")
        } else {
            rest.split_once('-')?
        };

        if [major, minor, patch, build]
            .iter()
            .any(|part| !digits_only(part))
        {
            return None;
        }

        let major: usize = major.parse().ok()?;
        let minor: usize = minor.parse().ok()?;
        let patch: usize = patch.parse().ok()?;
        let build: usize = build.parse().ok()?;

        Some(Self {
            major,
            minor,
            patch,
            build,
        })
    }

    fn bump(&self) -> Self {
        Self {
            build: self.build + 1,
            ..*self
        }
    }
}

impl std::fmt::Display for SemanticVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}-{}",
            self.major, self.minor, self.patch, self.build
        )
    }
}

fn digits_only(s: &str) -> bool {
    s.bytes().all(|byte| byte.is_ascii_digit())
}
