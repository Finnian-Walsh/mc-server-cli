use crate::{
    error::{Error, Result},
    server::{get_last_used, save_last_used},
    session,
};
use std::{
    collections::HashSet,
    ffi::OsStr,
    fmt::Display,
    io::{self, Read},
    path::Path,
    process::{Command, Stdio},
    thread,
    time::Duration,
};

pub static BASE_COMMAND: &str = "zellij";
pub static SUFFIX: &str = ".mcserver";

pub fn get_name(server: impl Display) -> String {
    format!("{server}{SUFFIX}")
}

fn get_alive_server_sessions() -> Result<HashSet<String>> {
    let output = Command::new(BASE_COMMAND).arg("list-sessions").output()?;

    match output.status.code() {
        Some(0) => Ok(String::from_utf8_lossy(&output.stdout)
            .to_string()
            .lines()
            .filter(|line| {
                let bracket_pos = match line.rfind('(') {
                    Some(pos) => pos,
                    None => return true,
                };

                !line[bracket_pos..].contains("EXITED") // if there is no "EXITED", still alive
            })
            .map(|line| {
                match line.rfind("[Created") {
                    Some(pos) => &line[7..=pos - 5],
                    None => &line[7..], // unexpected error
                }
                .to_string()
            })
            .filter(|session| session.ends_with(session::SUFFIX))
            .map(|session| session[..session.len() - session::SUFFIX.len()].to_string())
            .collect()),
        Some(1) => Ok(HashSet::new()), // no sessions
        _ => Err(Error::CommandFailure {
            code: output.status.code(),
            stderr: Some(output.stderr),
        }),
    }
}

fn push_last_used(server: &mut String) {
    let last_used = get_last_used(&server);

    server.push_str(" (Last used \x1b[35;1m");
    server.push_str(last_used.unwrap_or(None).as_deref().unwrap_or("unknown"));
    server.push_str("\x1b[0m ago)");
}

pub fn retain_active(servers: &mut Vec<String>) -> Result<()> {
    let sessions = get_alive_server_sessions()?;
    servers.retain(|server| sessions.contains(server));
    Ok(())
}

pub fn retain_inactive(servers: &mut Vec<String>) -> Result<()> {
    let sessions = get_alive_server_sessions()?;
    servers.retain(|server| !sessions.contains(server));
    servers.iter_mut().for_each(push_last_used);
    Ok(())
}

pub fn tag_servers(servers: &mut [String]) -> Result<()> {
    let sessions = get_alive_server_sessions()?;

    servers.iter_mut().for_each(|server| {
        if sessions.contains(server) {
            server.push_str(" (\x1b[32;1mactive\x1b[0m)");
        } else {
            push_last_used(server);
        }
    });
    Ok(())
}

pub fn attach(server: &str) -> Result<()> {
    let mut child = Command::new(BASE_COMMAND)
        .arg("attach")
        .arg(get_name(server))
        .stderr(Stdio::piped())
        .spawn()?;

    let status = child.wait()?;

    if status.success() {
        save_last_used(server)
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

pub fn new_session<S, I>(session: S, initial_command: Option<I>) -> Result<()>
where
    S: AsRef<OsStr>,
    I: AsRef<OsStr>,
{
    Command::new(BASE_COMMAND)
        .arg("delete-session")
        .arg(&session)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    let mut command = Command::new(BASE_COMMAND);
    command.arg("--session").arg(&session);
    let mut child = command.spawn()?;

    thread::sleep(Duration::from_millis(300));

    if let Some(command) = initial_command {
        write_line(&session, command)?;
    }

    child.wait()?;

    Ok(())
}

pub fn new_server(
    server: impl Display + AsRef<Path>,
    initial_command: Option<impl AsRef<OsStr>>,
) -> Result<()> {
    save_last_used(&server)?;
    let session_name = get_name(&server);
    new_session(session_name, initial_command)?;
    save_last_used(&server)
}

fn session_write(
    session: impl AsRef<OsStr>,
    mode: &'static str,
    chars: impl AsRef<OsStr>,
) -> Result<()> {
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

pub fn write_chars(session: impl AsRef<OsStr>, chars: impl AsRef<OsStr>) -> Result<()> {
    session_write(session, "write-chars", chars)
}

pub fn write_line(session: impl AsRef<OsStr>, chars: impl AsRef<OsStr>) -> Result<()> {
    write_chars(&session, chars)?;
    session_write(&session, "write", "13")?; // 13 is for carriage return
    Ok(())
}
