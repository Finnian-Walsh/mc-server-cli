use const_format::formatcp;
use std::{io, path::PathBuf, process::Command};

pub static REPO_OWNER: &str = "Finnian-Walsh";
pub static REPO_NAME: &str = "mc-server-cli";
pub static REPO_URL: &str = formatcp!("https://github.com/{}/{}", REPO_OWNER, REPO_NAME);

pub fn update_with_git(commit: Option<String>) -> io::Result<()> {
    Command::new("cargo")
        .arg("install")
        .arg("--git")
        .arg(if let Some(commit) = commit { format!("{}/commit/{}", REPO_URL, commit) } else { REPO_URL.to_string() })
        .arg("--force")
        .spawn()?
        .wait()?;

    Ok(())
}

pub fn update_with_path(path: PathBuf) -> io::Result<()> {
    Command::new("cargo")
        .arg("install")
        .arg("--path")
        .arg(path)
        .arg("--force")
        .spawn()?
        .wait()?;

    Ok(())
}
