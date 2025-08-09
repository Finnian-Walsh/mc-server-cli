use std::{fmt, io, process::Command};

fn get_sessions() -> io::Result<Vec<String>> {
    let output = Command::new("tmux")
        .arg("list-sessions")
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "tmux list-sessions command has failed"));
    }

    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| {
            line.find(':')
                .map(|pos| line[..pos].to_string())
        })
        .collect()
    )
}

#[derive(Debug)]
pub enum AttachError {
    Io(io::Error),
    SessionInexistent(String),
    TmuxFailure(String),
}

impl From<io::Error> for AttachError {
    fn from(err: io::Error) -> Self {
        AttachError::Io(err)
    }
}

impl fmt::Display for AttachError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttachError::Io(err) => write!(f, "IO error: {}", err),
            AttachError::SessionInexistent(session) => write!(f, "Session {} does not exist", session),
            AttachError::TmuxFailure(msg) => write!(f, "Tmux failure: {}", msg),
        }
    }   
}

impl std::error::Error for AttachError {}

pub fn attach(session: &str) -> Result<(), AttachError> {
    if !get_sessions()?.iter().any(|s| s == session) {
        return Err(AttachError::SessionInexistent(session.to_string()));
    }

    let mut child = Command::new("tmux")
        .arg("attach")
        .arg("-t")
        .arg(&session)
        .spawn()?;

    let status = child.wait()?;
            
    if status.success() {
        Ok(())
    } else {
        Err(AttachError::TmuxFailure("Failed to attach to session".to_string()))
    }
}

pub fn new(name: Option<&str>, process_command: Option<&str>) -> io::Result<()> {
    let mut command = Command::new("tmux");
    command.arg("new");

    if let Some(name) = name {
        command.arg("-s")
               .arg(name);
    }

    if let Some(process_command) = process_command {
        command.arg("bash")
               .arg("-c")
               .arg(process_command);
    }
    
    let mut child = command.spawn()?;
    let status = child.wait()?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Command failed"))
    }
}

