use const_format::formatcp;
use git2::{self, Repository};
use server::{
    config,
    executables::{self, Executable},
    home,
};
use std::{
    env, fmt, io, path,
    process::{Command, Stdio},
    result,
};

static REPO_OWNER: &str = "Finnian-Walsh";
static REPO_NAME: &str = "mc-server-cli";
static GITHUB_REPO_URL: &str = formatcp!("https://github.com/{}/{}", REPO_OWNER, REPO_NAME);

#[derive(Debug)]
pub enum Error {
    BuildFailure,
    Git2(git2::Error),
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::BuildFailure => write!(f, "cargo build failed"),
            Error::Git2(err) => write!(f, "{}", err),
            Error::Io(e) => write!(f, "{}", e),
        }
    }
}

impl From<git2::Error> for Error {
    fn from(err: git2::Error) -> Self {
        Error::Git2(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl std::error::Error for Error {}

pub type Result<T> = result::Result<T, Error>;

macro_rules! build_binaries {
    ( $func:expr, $( $arg:expr ),* $(,)? ) => {
        {
            let mut command = Command::new("cargo");
            command.arg("build");

            $func(&mut command);
            $(
                command.arg("--bin").arg($arg);
            )*

            command.status()
        }
    };

    ( $( $arg:expr ),* $(,)? ) => {
        build_binaries!(|_: &mut Command| {}, $( $arg ),*)
    };
}

pub fn local_update() -> Result<()> {
    if !build_binaries!("server")?.success() {
        return Err(Error::BuildFailure);
    }

    if !build_binaries!(
        |c: &mut Command| {
            c.arg("--release");
        },
        "server"
    )?
    .success()
    {
        return Err(Error::BuildFailure);
    }

    let exec = executables::get(Executable::Server, "release");
    let path = home::get()?.join(config::get("path")?).join("server");

    executables::sudo_update(exec, path)?;

    Ok(())
}

pub fn remote_update() -> Result<()> {
    let _path = &env::temp_dir().join("mc-server-cli");

    Ok(())
}

pub fn local_self_update() -> Result<()> {
    build_binaries!(
        |c: &mut Command| {
            c.arg("--release");
        },
        "serverup",
        "serverup_updater"
    )?;

    Command::new(format!(
        ".{}{}",
        path::MAIN_SEPARATOR_STR,
        executables::get(Executable::ServerupUpdater, "release")
    ))
    .stdin(Stdio::inherit())
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .spawn()?;

    Ok(())
}

pub fn remote_self_update() -> Result<()> {
    let path = &env::temp_dir().join(REPO_NAME);
    Repository::clone(GITHUB_REPO_URL, path)?;

    build_binaries!(
        |c: &mut Command| {
            c.arg("--release");
            c.current_dir(&path);
        },
        "serverup",
        "serverup_updater"
    )?;

    Command::new(format!(
        ".{}{}",
        path::MAIN_SEPARATOR_STR,
        executables::get(Executable::ServerupUpdater, "release")
    ))
    .spawn()?;

    Ok(())
}
