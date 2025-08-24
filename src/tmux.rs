use crate::{config, home};
use std::{
    collections::HashSet,
    ffi::OsStr,
    fmt, fs,
    io::{self, Read},
    process::{Command, Stdio},
    result,
};

fn get_sessions() -> io::Result<HashSet<String>> {
    let output = Command::new("tmux").arg("list-sessions").output()?;

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "tmux list-sessions command has failed",
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| line.find(':').map(|pos| line[..pos].to_string()))
        .collect())
}

pub fn get_servers() -> io::Result<Vec<String>> {
    let mut servers = vec![];

    for entry in fs::read_dir(home::get()?.join(config::get("servers")?))? {
        let entry = entry?;
        servers.push(entry.file_name().to_string_lossy().into_owned());
    }

    Ok(servers)
}

pub fn get_active_servers() -> io::Result<Vec<String>> {
    let sessions = get_sessions()?;
    let mut servers = get_servers()?;
    servers.retain(|server| sessions.contains(server));
    Ok(servers)
}

pub fn get_inactive_servers() -> io::Result<Vec<String>> {
    let sessions = get_sessions()?;
    let mut servers = get_servers()?;
    servers.retain(|server| !sessions.contains(server));
    Ok(servers)
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    TmuxFailure { code: Option<i32>, stderr: String },
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "{}", err),
            Error::TmuxFailure { code, stderr } => write!(
                f,
                "Tmux failed with code {}: {}",
                code.map(|c| c.to_string())
                    .unwrap_or_else(|| "none".to_string()),
                stderr
            ),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = result::Result<T, Error>;

pub fn attach(session: &str) -> Result<()> {
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
        let mut stderr = String::new();
        child
            .stderr
            .take()
            .ok_or(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "Failed to take stderr pipe",
            ))?
            .read_to_string(&mut stderr)?;

        Err(Error::TmuxFailure {
            code: status.code(),
            stderr,
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
