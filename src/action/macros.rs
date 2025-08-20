use super::Cmd;

pub(crate) trait CmdAppend<T> {
    fn append(&mut self, data: T);
}

impl CmdAppend<String> for Cmd {
    fn append(&mut self, s: String) {
        self.args.push(s);
    }
}

impl CmdAppend<Vec<String>> for Cmd {
    fn append(&mut self, strings: Vec<String>) {
        for s in strings {
            self.append(s);
        }
    }
}

impl CmdAppend<&str> for Cmd {
    fn append(&mut self, s: &str) {
        self.append(s.to_string());
    }
}

impl CmdAppend<Vec<&str>> for Cmd {
    fn append(&mut self, strings: Vec<&str>) {
        for s in strings {
            self.append(s);
        }
    }
}

macro_rules! cmd {
    ($exe:expr) => {{
        $crate::action::Action::Cmd($crate::action::Cmd::new($exe))
    }};

    ($exe:expr, $($x:expr),+ $(,)?) => {{
        let mut cmd = $crate::action::Cmd { exe: $exe.into(), args: vec![] };
        use $crate::action::macros::CmdAppend;
        $(cmd.append($x);)+
        $crate::action::Action::Cmd(cmd)
    }};
}
pub(crate) use cmd;

macro_rules! cwd {
    ($dir:expr) => {
        $crate::action::Action::Cwd($crate::action::Cwd { dir: $dir.into() })
    };
}
pub(crate) use cwd;

macro_rules! write_file {
    ($path:expr, $($arg:tt)*) => {
        $crate::action::Action::WriteFile(
            $crate::action::WriteFile {
                path: $path.into(),
                contents: format!("{}", format_args!($($arg)*))
            }
        )
    };
}
pub(crate) use write_file;
