use const_format::formatcp;
use git2::{self, Repository};
use server::{
    executables::{self, Executable},
    home,
};
use std::{env, fmt, fs, io, path, process::Command, result};

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
    ( $func:expr, $( $arg:expr ),* ) => {
        {
            let mut command = Command::new("cargo");
            command.arg("build");

            $func(&mut command);
            $(
                command.args(["--bin", $arg]);
            )*

            command.status()
        }
    };
}

pub fn local_update() -> Result<()> {
    println!("building debug...");
    if !build_binaries!(|_| {}, "server")?.success() {
        return Err(Error::BuildFailure);
    }

    println!("building release...");
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

    println!("{}", executables::get(Executable::Server, "release"));

    fs::copy(
        executables::get(Executable::Server, "release"),
        home::get()?.join(".local").join("bin").join("server"),
    )?;

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
