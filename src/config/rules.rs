use crate::toml::TomlValueWithPath;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct Rules {
    pub(crate) map: HashMap<String, Vec<String>>,
}

impl Rules {
    pub(crate) fn from_toml(toml: TomlValueWithPath) -> Self {
        let map = toml
            .into_table()
            .iter()
            .map(|(k, v)| (k, v.into_array_of_strings()))
            .collect::<HashMap<_, _>>();
        Self { map }
    }

    pub(crate) fn render(mut self) -> String {
        let mut makefile = Makefile::new();

        if let Some(lines) = self.map.remove("%") {
            makefile.push("%", lines);
        }

        makefile.push("override_dh_auto_test", ["@true"]);
        makefile.push("override_dh_strip", ["@true"]);

        for (target, lines) in self.map {
            makefile.push(target, lines);
        }

        makefile.done()
    }
}

struct Makefile {
    buf: String,
}

impl Makefile {
    fn new() -> Self {
        Self {
            buf: ["#!/usr/bin/make -f\n", "export DH_VERBOSE = 1\n"].join(""),
        }
    }

    fn push(
        &mut self,
        target: impl AsRef<str>,
        lines: impl IntoIterator<Item = impl Into<String>>,
    ) {
        self.buf.push_str(&format!("{}:\n", target.as_ref()));
        for line in lines {
            let line: String = line.into();
            self.buf.push_str(&format!("\t{line}\n"));
        }
    }

    fn done(self) -> String {
        self.buf
    }
}
