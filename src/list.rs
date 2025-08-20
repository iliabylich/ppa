use std::path::PathBuf;

pub(crate) struct List;

impl List {
    pub(crate) fn read(path: PathBuf) -> Vec<PathBuf> {
        let list = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| error!(err = err, "failed to open {path:?}"));
        let dir = path
            .parent()
            .unwrap_or_else(|| error!("failed to get parent of {path:?}"));

        list.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(|filename| dir.join(filename))
            .collect()
    }
}
