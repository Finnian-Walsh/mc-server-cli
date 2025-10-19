use crate::{
    config::{get_current_server_directory, get_expanded_servers_dir},
    error::{Error, Result},
    platforms::Platform,
};
use reqwest::{blocking, header};
use std::{
    cell::OnceCell,
    collections::HashSet,
    env,
    fmt::{self, Display, Formatter},
    fs::{self, File},
    io::{self, Write},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};
use url::Url;

pub struct ServerObject {
    pub name: String,
    pub tags: Vec<String>,
}

impl ServerObject {
    pub fn new(name: String) -> Self {
        ServerObject { name, tags: vec![] }
    }
}

impl Display for ServerObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        for tag in &self.tags {
            write!(f, " {tag}")?;
        }
        Ok(())
    }
}

fn copy_jar(server_dir: impl AsRef<Path>, file_name: String, mut jar: impl io::Read) -> Result<()> {
    env::set_current_dir(server_dir)?;

    let mut jar_file = File::create(&file_name)?;
    io::copy(&mut jar, &mut jar_file)?;

    let mut jarfile_txt = File::create("jarfile.txt")?;
    writeln!(jarfile_txt, "{file_name}")?;

    Ok(())
}

pub fn copy_directory(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            copy_directory(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}

pub fn remove_dir_with_retries(dir: impl AsRef<Path>) -> Result<()> {
    const ATTEMPTS: u8 = 10;

    for i in 1..=ATTEMPTS {
        if let Err(err) = fs::remove_dir_all(&dir) {
            if i == ATTEMPTS {
                return Err(Error::Io(err));
            }
        } else {
            return Ok(());
        }
    }

    unreachable!("Code returns before the for loop ends")
}

fn remove_server(server: String) -> Result<()> {
    remove_dir_with_retries(get_expanded_servers_dir()?.join(server))?;
    Ok(())
}

pub fn remove_servers(servers: Vec<String>) -> Result<()> {
    let all_servers = get_all_hashed()?;

    for server in servers {
        let server = if server == "." {
            get_current_server_directory()?
        } else {
            server
        };

        if all_servers.get(&server).as_ref().is_none() {
            return Err(Error::ServerNotFound(server));
        }

        remove_server(server)?;
    }

    Ok(())
}

pub fn remove_servers_with_confirmation(servers: Vec<String>) -> Result<()> {
    let all_servers = get_all_hashed()?;

    for server in servers {
        if all_servers.get(&server).as_ref().is_none() {
            return Err(Error::ServerNotFound(server));
        }

        if loop {
            print!("Enter `{server}` to delete the server or nothing to cancel operation: ");
            io::stdout().flush()?;

            let mut response = String::new();
            io::stdin().read_line(&mut response)?;

            if server == response.trim_end() {
                break true;
            } else if response.is_empty() {
                break false;
            }
        } {
            remove_server(server)?;
            println!("Server successfully removed");
        } else {
            println!("Operation canceled");
        }
    }

    Ok(())
}

pub fn init(download_url: Url, platform: Platform, name: Option<String>) -> Result<()> {
    let fallback_name: OnceCell<String> = OnceCell::new();
    let get_fallback_name = || fallback_name.get_or_init(|| format!("{platform:?}").to_lowercase());

    let name = name.unwrap_or_else(|| format!("{:?}-server", get_fallback_name()));
    let servers_dir = &get_expanded_servers_dir()?;

    let mut server_dir = servers_dir.join(&name);

    if server_dir.exists() {
        let mut number = 2;

        server_dir = loop {
            let dir = servers_dir.join(format!("{}-{}", &name, number));

            if !dir.exists() {
                break dir;
            }
            number += 1;
        }
    }

    fs::create_dir_all(&server_dir)?;

    println!("Downloading from {download_url}...");
    let response = blocking::get(download_url)?;

    let file_name = response
        .headers()
        .get(header::CONTENT_DISPOSITION)
        .map(|disposition| disposition.to_str())
        .transpose()?
        .and_then(|cd| cd.split("filename=\"").nth(1))
        .and_then(|slice| slice.split('"').next())
        .map(String::from)
        .unwrap_or_else(|| format!("{}.jar", get_fallback_name()));

    if let Err(err) = copy_jar(&server_dir, file_name, response) {
        remove_dir_with_retries(server_dir)?;
        return Err(err);
    }

    Ok(())
}

pub fn for_each(mut f: impl FnMut(String)) -> Result<()> {
    let servers_dir = get_expanded_servers_dir()?;

    if !servers_dir.exists() || !servers_dir.is_dir() {
        return Err(Error::MissingDirectory {
            dir: servers_dir.to_path_buf(),
        });
    }

    for entry in fs::read_dir(servers_dir)? {
        let entry = entry?;
        let file_name = entry.file_name().to_string_lossy().to_string();
        f(file_name);
    }

    Ok(())
}

pub fn get_all_hashed() -> Result<HashSet<String>> {
    let mut servers = HashSet::new();
    for_each(|s| {
        servers.insert(s);
    })?;
    Ok(servers)
}

static LAST_USED_FILE: &str = "last_used.timestamp";

pub fn get_last_used(server: impl AsRef<Path>) -> Result<Option<String>> {
    let timestamp_path = get_expanded_servers_dir()?
        .join(&server)
        .join(LAST_USED_FILE);

    if !timestamp_path.exists() {
        return Ok(None);
    }

    let data = fs::read(timestamp_path)?;

    if data.len() != 8 {
        return Err(Error::InvalidTimestampFile(
            server.as_ref().to_string_lossy().to_string(),
        ));
    }

    let bytes: [u8; 8] = data
        .try_into()
        .map_err(|_| Error::InvalidTimestampFile(server.as_ref().to_string_lossy().to_string()))?;

    let timestamp = u64::from_le_bytes(bytes);

    let now_ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| Error::TimeWentBackwards)?
        .as_secs();

    let difference = now_ts.saturating_sub(timestamp);

    const SECS_MINUTE: u64 = 60;
    const SECS_HOUR: u64 = SECS_MINUTE * 60;
    const SECS_DAY: u64 = SECS_HOUR * 24;
    const SECS_YEAR: u64 = (SECS_DAY as f64 * 365.2425) as u64;

    let years = difference / SECS_YEAR;
    let years_remainder = difference % SECS_YEAR;

    let days = years_remainder / SECS_DAY;
    let days_remainder = years_remainder % SECS_DAY;

    let hours = days_remainder / SECS_HOUR;
    let hours_remainder = days_remainder % SECS_HOUR;

    let minutes = hours_remainder / SECS_MINUTE;
    let seconds = hours_remainder % SECS_MINUTE;

    if years > 0 {
        Ok(Some(format!(
            "{years}y {days}d {hours}h {minutes}m {seconds}s"
        )))
    } else if days > 0 {
        Ok(Some(format!("{days}d {hours}h {minutes}m {seconds}s")))
    } else if hours > 0 {
        Ok(Some(format!("{hours}h {minutes}m {seconds}s")))
    } else if minutes > 0 {
        Ok(Some(format!("{minutes}m {seconds}s")))
    } else {
        Ok(Some(format!("{seconds}s")))
    }
}

pub fn save_last_used(server: impl AsRef<Path>) -> Result<()> {
    let now = SystemTime::now();

    let timestamp = now
        .duration_since(UNIX_EPOCH)
        .map_err(|_| Error::TimeWentBackwards)?
        .as_secs();

    let timestamp_path = get_expanded_servers_dir()?
        .join(&server)
        .join(LAST_USED_FILE);
    let mut file = File::create(timestamp_path)?;
    file.write_all(&timestamp.to_le_bytes())?;

    Ok(())
}
