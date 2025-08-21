use std::process::Command;

pub(crate) fn num_cpus() -> u8 {
    let stdout = Command::new("nproc")
        .output()
        .unwrap_or_else(|err| error!(err = err, "failed to call nproc"))
        .stdout;

    String::from_utf8(stdout)
        .unwrap_or_else(|err| error!(err = err, "non-utf-8 output of nproc"))
        .trim()
        .parse()
        .unwrap_or_else(|err| error!(err = err, "non-numeric output of nproc"))
}
