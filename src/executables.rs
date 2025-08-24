use std::{
    ffi::OsStr,
    io, path,
    process::{Command, ExitStatus, Stdio},
};

pub enum Executable {
    Server,
    Serverup,
    ServerupUpdater,
}

pub fn get(executable: Executable, build_mode: &str) -> String {
    format!(
        "target{}{}{}{}",
        path::MAIN_SEPARATOR_STR,
        build_mode,
        path::MAIN_SEPARATOR_STR,
        match executable {
            Executable::Server => "server",
            Executable::Serverup => "serverup",
            Executable::ServerupUpdater => "serverup_updater",
        }
    )
}

pub fn sudo_update<T, U>(src: T, dst: U) -> io::Result<ExitStatus>
where
    T: AsRef<OsStr>,
    U: AsRef<OsStr>,
{
    let mut child = Command::new("sudo")
        .arg("-S")
        .arg("cp")
        .arg(src)
        .arg(dst)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    Ok(child.wait()?)
}
