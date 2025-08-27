use crate::{config, home};
use std::{
    collections::HashSet,
    ffi::OsStr,
    fmt, fs,
    io::{self, Read},
    process::{Command, Stdio},
    result,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    Io(#[from] io::Error),

    TmuxFailure { code: Option<i32>, stderr: Vec<u8> },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "{}", err),
            Self::TmuxFailure { code, stderr } => write!(
                f,
                "Tmux failed with code {}: {}",
                code.map(|c| c.to_string())
                    .unwrap_or_else(|| String::from("none")),
                String::from_utf8_lossy(stderr)
            ),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;

fn get_sessions() -> Result<HashSet<String>> {
    let output = Command::new("tmux").arg("list-sessions").output()?;

    if !output.status.success() {
        return Err(Error::TmuxFailure {
            code: output.status.code(),
            stderr: output.stderr,
        });
    }

    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| line.find(':').map(|pos| line[..pos].to_string()))
        .collect())
}

pub fn get_servers() -> Result<Vec<String>> {
    let mut servers = vec![];

    for entry in fs::read_dir(home::get()?.join(config::get("servers")?))? {
        let entry = entry?;
        servers.push(entry.file_name().to_string_lossy().into_owned());
    }

    Ok(servers)
}

pub fn get_active_servers() -> Result<Vec<String>> {
    let sessions = get_sessions()?;
    let mut servers = get_servers()?;
    servers.retain(|server| sessions.contains(server));
    Ok(servers)
}

pub fn get_inactive_servers() -> Result<Vec<String>> {
    let sessions = get_sessions()?;
    let mut servers = get_servers()?;
    servers.retain(|server| !sessions.contains(server));
    Ok(servers)
}

pub fn attach(session: &str) -> Result<()> {
    if session == "." {}

    let mut child = Command::new("tmux")
        .arg("attach")
        .arg("-t")
        .arg(&session)
        .stderr(Stdio::piped())
        .spawn()?;

    let status = child.wait()?;

    if status.success() {
        Ok(())
    } else {
        let mut buf = Vec::new();
        child
            .stderr
            .take()
            .ok_or(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "Failed to take stderr pipe",
            ))?
            .read_to_end(&mut buf)?;

        Err(Error::TmuxFailure {
            code: status.code(),
            stderr: buf,
        })
    }
}

pub fn new(name: Option<&str>, process_command: Option<&str>) -> io::Result<()> {
    let mut command = Command::new("tmux");
    command.arg("new");

    if let Some(name) = name {
        command.arg("-s").arg(name);
    }

    if let Some(process_command) = process_command {
        command.arg("bash").arg("-c").arg(process_command);
    }

    let mut child = command.spawn()?;
    let status = child.wait()?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Command failed"))
    }
}

pub fn execute<S: AsRef<OsStr>, C: AsRef<OsStr>>(name: S, command: C) -> io::Result<()> {
    let status = Command::new("tmux")
        .arg("send-keys")
        .arg("-t")
        .arg(name)
        .arg(command)
        .arg("Enter")
        .status()?;

    if !status.success() {}

    Ok(())
}
