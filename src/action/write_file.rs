#[derive(Debug)]
pub(crate) struct WriteFile {
    pub(crate) path: String,
    pub(crate) contents: String,
}

impl WriteFile {
    pub(crate) fn exec(&self) {
        std::fs::write(&self.path, &self.contents)
            .unwrap_or_else(|err| error!(err = err, "Failed to write to {}", self.path))
    }

    pub(crate) fn explain(&self) {
        yellow!("Writing to {}:\n{}", self.path, self.contents)
    }
}
