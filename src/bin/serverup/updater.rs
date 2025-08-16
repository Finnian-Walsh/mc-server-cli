use git2::{self, Repository};
use server::{config, home};

#[derive(Debug)]
pub enum Error {
    BuildFailure,
    Git2(git2::Error),
    Home(home::Error),
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::BuildFailure => write!(f, "cargo build failed"),
            Error::Git2(err) => write!(f, "{}", err),
            Error::Home(err) => write!(f, "{}", err),
            Error::Io(e) => write!(f, "{}", e),
        }
    }
}

impl From<git2::Error> for Error {
    fn from(err: git::Error) -> Self {
        Error::Git2(err)
    }
}

impl From<home::Error> for Error {
    fn from(err: home::Error) -> Self {
        Error::Home(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl std::error::Error for Error {}

pub type Result<T> = result::Result<T, Error>;

enum Executable {
    Server,
    Serverup,
    ServerupUpdater,
}

const fn get_executable(executable: Executable, mode: &str) -> &Path {
    Path::new(concat!("target", mode, match executable {
        Executable::Server => "server",
        Executable::Serverup => "serverup",
        Executable::ServerupUpdater => "serverup_updater",
    }))
}

fn build_binaries<I, S, F>(binaries: I, mutator: F) -> Result<ExitStatus>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    F: FnMut(&mut Command),
{
    let mut command = Command::new("cargo").arg("build");
    
    mutator(&mut command);

    for target in binaries {
        command.args(["--bin", target]);
    }

    Ok(command.status()?)
}

pub fn update() -> Result<()> {
    if !build_binaries(&["server"], |_|)?.status.success() {
        return Err(Error::BuildFailure);
    }

    if !build_binaries(&["server"], |c| c.arg("--release"))?.status.success() {
        return Err(Error::BuildFailure);
    }

    fs::copy(
        "target/release/server",
        home::get().join(".local").join("bin").join("server"),
    )?;

    Ok(())
}

fn local_self_update() -> Result<()> {
    build_binaries(&["serverup", "serverup_updater"], |c| c.arg("--release"))?;

    Command::new("./target/release/serverup_updater").spawn()?;

    Ok(())
}

fn remote_self_update() -> Result<()> {
    let path = &env::temp_dir().join("server-cli");
    let repo = Repository::clone("https://github.com/Finnian-Walsh/server-cli.git")?;

    build_binaries(&["serverup", "serverup_updater"], |c| {
        c.arg("--release");
        c.current_dir(&path);
    })?;
    
    Command::new("./
}

