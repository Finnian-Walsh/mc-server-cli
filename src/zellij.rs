use crate::error::{Error, Result};
use std::{
    collections::HashSet,
    ffi::OsStr,
    fmt,
    io::{self, Read},
    process::{Command, Stdio},
};

static BASE_COMMAND: &str = "zellij";

fn get_sessions() -> Result<HashSet<String>> {
    let output = Command::new(BASE_COMMAND)
        .arg("list-sessions")
        .arg("--short")
        .output()?;

    if !output.status.success() {
        return Err(Error::CommandFailure {
            code: output.status.code(),
            stderr: Some(output.stderr),
        });
    }

    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|l| l.to_string())
        .collect())
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
        } else {
            println!("nope");
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
    let mut command = Command::new(BASE_COMMAND);

    command.arg("--session").arg(&name);

    let mut child = command.spawn()?;
    let status = child.wait()?;

    if !status.success() {
        return Err(Error::CommandFailure {
            code: status.code(),
            stderr: None,
        });
    }

    if let Some(command) = initial_command {
        write_chars(name, command)?;
    }

    Ok(())
}

pub fn write_chars<S, C>(session: S, chars: C) -> Result<()>
where
    S: AsRef<OsStr>,
    C: AsRef<OsStr>,
{
    let status = Command::new(BASE_COMMAND)
        .arg("--session")
        .arg(session)
        .arg("action")
        .arg("write-chars")
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

pub fn write_line<S, C>(session: S, chars: C) -> Result<()>
where
    S: AsRef<OsStr>,
    C: AsRef<OsStr> + fmt::Display,
{
    write_chars(session, format!("{}\r", chars))?;
    Ok(())
}
