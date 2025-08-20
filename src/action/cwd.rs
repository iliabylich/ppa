#[derive(Debug)]
pub(crate) struct Cwd {
    pub(crate) dir: String,
}

impl Cwd {
    pub(crate) fn exec(&self) {
        std::env::set_current_dir(&self.dir).unwrap_or_else(|err| {
            error!(
                err = err,
                "failed to change working directory to {}", self.dir
            )
        })
    }

    pub(crate) fn explain(&self) {
        green!("cwd {}", self.dir)
    }
}
