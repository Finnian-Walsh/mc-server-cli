use std::{io, process::Command};

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

pub enum AttachError {
    Io(io::Error),
    SessionInexistent,
    TmuxFailure(String),
}

impl From<io::Error> for AttachError {
    fn from(e: io::Error) -> Self {
        AttachError::Io(e)
    }
}

pub fn attach(session: &str) -> Result<(), AttachError> {
    if !get_sessions()?.iter().any(|s| s == session) {
        return Err(AttachError::SessionInexistent);
    }

    let mut child = Command::new("tmux")
        .arg("attach")
        .arg("-t")
        .arg(&session)
        .spawn()?;

    let output = child.wait()?;
            
    if output.success() {
        Ok(())
    } else {
        Err(AttachError::TmuxFailure("Attach failed".to_string()))
    }
}

