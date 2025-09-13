use build_deb_package::{
    args::paths_from_args, config::Config, github::GitHub, green, red, yellow,
};
use std::{collections::VecDeque, sync::Mutex};

fn main() {
    let sources = paths_from_args()
        .into_iter()
        .map(Config::read)
        .filter_map(|config| {
            let config_path = config.relative_file_path();
            let Some((user, repo)) = config.git_user_and_repo() else {
                yellow!("[{config_path}] skipping (not git user/repo)");
                return None;
            };
            let Some(branch_or_tag) = config.git_branch_or_tag() else {
                yellow!("[{config_path}] skipping (not git branch_or_tag)");
                return None;
            };
            Some((config_path, user, repo, branch_or_tag))
        })
        .map(|(config_path, user, repo, branch_or_tag)| GitSource {
            config_path,
            user,
            repo,
            branch_or_tag,
        })
        .collect::<VecDeque<_>>();

    let queue = Mutex::new(sources);
    let outputs = Mutex::new(vec![]);

    std::thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                loop {
                    let source = {
                        let mut queue = queue.lock().unwrap();
                        queue.pop_front()
                    };
                    let Some(source) = source else {
                        break;
                    };

                    let output = source.check();
                    {
                        let mut outputs = outputs.lock().unwrap();
                        outputs.push(output);
                    }
                }
            });
        }
    });

    let mut outputs = outputs.into_inner().unwrap();
    outputs.sort_unstable_by_key(|result| result.user_slash_repo());
    for output in outputs {
        output.print();
    }
}

#[derive(Debug)]
struct GitSource {
    config_path: String,
    user: String,
    repo: String,
    branch_or_tag: String,
}

impl GitSource {
    fn latest_remote_version(&self) -> Result<String, String> {
        let github = GitHub::new(&self.user, &self.repo);

        match github.latest_release() {
            Ok(release) => Ok(release),
            Err(release_check_err) => match github.latest_tag() {
                Ok(tag) => Ok(tag),
                Err(tag_check_err) => Err(format!(
                    "release check failed: {release_check_err}\ntag check failed: {tag_check_err}"
                )),
            },
        }
    }

    fn check(self) -> Output {
        let up_to_date = self
            .latest_remote_version()
            .map(|remote_version| (remote_version == self.branch_or_tag, remote_version));

        Output {
            config_path: self.config_path,
            user: self.user,
            repo: self.repo,
            local_version: self.branch_or_tag,
            up_to_date,
        }
    }
}

struct Output {
    config_path: String,
    user: String,
    repo: String,
    local_version: String,
    up_to_date: Result<(bool, String), String>,
}

impl Output {
    fn user_slash_repo(&self) -> String {
        format!("{}/{}", self.user, self.repo)
    }

    fn print(self) {
        let Self {
            config_path,
            local_version,
            up_to_date,
            ..
        } = self;

        match up_to_date {
            Ok((true, remote_version)) => {
                green!("[{config_path}] up to date: {local_version} vs {remote_version}")
            }
            Ok((false, remote_version)) => {
                red!("[{config_path}] outdated: {local_version} vs {remote_version}")
            }
            Err(err) => {
                red!("[{config_path}] failed to load info:\n{err}")
            }
        }
    }
}
