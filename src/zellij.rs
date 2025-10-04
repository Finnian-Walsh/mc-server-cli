use crate::error::{Error, Result};
use std::{
    collections::HashSet,
    ffi::OsStr,
    io::{self, Read},
    process::{Command, Stdio},
    thread,
    time::Duration,
};

static BASE_COMMAND: &str = "zellij";

fn get_sessions() -> Result<HashSet<String>> {
    let output = Command::new(BASE_COMMAND)
        .arg("list-sessions")
        .arg("--short")
        .output()?;

    match output.status.code() {
        Some(0) => Ok(String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|l| l.to_string())
            .collect()),
        Some(1) => Ok(HashSet::new()),
        _ => Err(Error::CommandFailure {
            code: output.status.code(),
            stderr: Some(output.stderr),
        }),
    }
}

pub fn retain_active(servers: &mut Vec<String>) -> Result<()> {
    let sessions = get_sessions()?;
    servers.retain(|server| sessions.contains(server));
    Ok(())
}

pub fn retain_inactive(servers: &mut Vec<String>) -> Result<()> {
    let sessions = get_sessions()?;
    servers.retain(|server| !sessions.contains(server));
    Ok(())
}

pub fn tag_active(servers: &mut Vec<String>) -> Result<()> {
    let sessions = get_sessions()?;

    servers.iter_mut().for_each(|server| {
        if sessions.contains(server) {
            server.push_str(" (active)");
        }
    });
    Ok(())
}

pub fn attach<S: AsRef<OsStr> + for<'a> PartialEq<&'a str>>(session: S) -> Result<()> {
    if session == "." {}

    let mut child = Command::new(BASE_COMMAND)
        .arg("attach")
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

        Err(Error::CommandFailure {
            code: status.code(),
            stderr: Some(buf),
        })
    }
}

pub fn new<N: AsRef<OsStr>, C: AsRef<OsStr>>(name: N, initial_command: Option<C>) -> Result<()> {
    Command::new(BASE_COMMAND)
        .arg("delete-session")
        .arg(&name)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    let mut command = Command::new(BASE_COMMAND);

    command.arg("--session").arg(&name);

    let mut child = command.spawn()?;

    thread::sleep(Duration::from_millis(300));

    if let Some(command) = initial_command {
        write_line(&name, command)?;
    }

    child.wait()?;

    Ok(())
}

fn session_write<S, C>(session: S, mode: &'static str, chars: C) -> Result<()>
where
    S: AsRef<OsStr>,
    C: AsRef<OsStr>,
{
    let status = Command::new(BASE_COMMAND)
        .arg("--session")
        .arg(session)
        .arg("action")
        .arg(mode)
        .arg(chars)
        .spawn()?
        .wait()?;

    if !status.success() {
        return Err(Error::CommandFailure {
            code: status.code(),
            stderr: None,
        });
    }

    Ok(())
}

pub fn write_chars<S, C>(session: S, chars: C) -> Result<()>
where
    S: AsRef<OsStr>,
    C: AsRef<OsStr>,
{
    session_write(session, "write-chars", chars)
}

pub fn write_line<S, C>(session: S, chars: C) -> Result<()>
where
    S: AsRef<OsStr>,
    C: AsRef<OsStr>,
{
    write_chars(&session, chars)?;
    session_write(&session, "write", "13")?;
    Ok(())
}

pub fn kill_session<S: AsRef<OsStr>>(session: S) -> Result<()> {
    let status = Command::new(BASE_COMMAND)
        .arg("kill-session")
        .arg(session)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(Error::CommandFailure {
            code: status.code(),
            stderr: None,
        })
    }
}
