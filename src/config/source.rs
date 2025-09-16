use crate::{error, toml::TomlValueWithPath};

#[derive(Debug)]
pub(crate) enum Source {
    None,
    GitClone(GitClone),
}

impl Source {
    pub(crate) fn from_toml(toml: TomlValueWithPath) -> Self {
        if let Some(string) = toml.try_to_string()
            && string == "none"
        {
            Self::None
        } else if let Some(table) = toml.try_to_table()
            && let Some(git_clone) = table.try_enter("git-clone")
        {
            Self::GitClone(GitClone::from_toml(git_clone))
        } else {
            error!("source must be either \"none\" or git-clone table")
        }
    }
}

#[derive(Debug)]
pub(crate) struct GitClone {
    pub(crate) url: String,
    pub(crate) branch_or_tag: String,
    pub(crate) post_clone_scripts: Vec<PostCloneScript>,
}

impl GitClone {
    fn from_toml(toml: TomlValueWithPath) -> Self {
        let table = toml.into_table();

        let url = table.enter("url").into_string();

        let branch_or_tag = table.enter("branch-or-tag").into_string();

        let post_clone_scripts = table
            .try_enter("post-clone-scripts")
            .map(|v| v.into_array())
            .unwrap_or_default()
            .into_iter()
            .map(|item| item.into_string())
            .map(PostCloneScript::parse)
            .collect::<Vec<_>>();

        Self {
            url,
            branch_or_tag,
            post_clone_scripts,
        }
    }
}

#[derive(Debug)]
pub(crate) struct PostCloneScript {
    pub(crate) exe: String,
    pub(crate) args: Vec<String>,
}

impl PostCloneScript {
    fn parse(line: String) -> Self {
        let mut parts = line.split(" ");
        let exe = parts
            .next()
            .unwrap_or_else(|| error!("script can't be empty"))
            .to_string();
        let args = parts.map(|part| part.to_string()).collect::<Vec<_>>();

        Self { exe, args }
    }
}
