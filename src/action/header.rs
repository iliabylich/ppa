#[derive(Debug)]
pub(crate) struct Header {
    pub(crate) text: String,
}

impl Header {
    pub(crate) fn explain(&self) {
        eprintln!("==== {} ====", self.text)
    }
}
