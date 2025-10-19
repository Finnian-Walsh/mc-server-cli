use crate::error::{Error, GlobalMutex, Result};
use std::{
    env, fs,
    ops::Deref,
    path::{Path, PathBuf},
    sync::{Mutex, MutexGuard, OnceLock},
};

mod config_defs {
    include!("../config_defs.rs");
}

use config_defs::{DynamicConfig, StaticConfig, Password, RconConfig};

include!(concat!(env!("OUT_DIR"), "/generated_cfg.rs"));

pub struct AutoConfig {
    // type ValueType = OnceLock<Mutex<DynamicConfig>>;
    value: OnceLock<Mutex<DynamicConfig>>,
    initial_value: OnceLock<DynamicConfig>,
}

impl AutoConfig {
    const fn new() -> Self {
        Self {
            value: OnceLock::new(),
            initial_value: OnceLock::new(),
        }
    }

    pub fn write(&self) -> Result<()> {
        let Some(mutex) = self.get() else {
            return Ok(());
        };

        let guard = mutex
            .lock()
            .map_err(|_| Error::GlobalMutexPoisoned(GlobalMutex::Config))?;

        if let Some(initial_value) = self.initial_value.get() {
            if *initial_value == *guard {
                return Ok(());
            }
        } else {
            eprintln!("Initial configuration value not set");
        }

        fs::create_dir_all(get_config_directory()?)?;
        fs::write(get_config_file()?, toml::to_string(&*guard)?)?;
        Ok(())
    }
}

impl Deref for AutoConfig {
    type Target = OnceLock<Mutex<DynamicConfig>>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

pub static CONFIG: AutoConfig = AutoConfig::new();

static CONFIG_DIRECTORY: OnceLock<PathBuf> = OnceLock::new();
static CONFIG_FILE: OnceLock<PathBuf> = OnceLock::new();

static EXPANDED_SERVERS_DIR: OnceLock<PathBuf> = OnceLock::new();

fn get_config_directory() -> Result<&'static Path> {
    if let Some(path) = CONFIG_DIRECTORY.get() {
        return Ok(path.as_path());
    }

    let path = shellexpand::full(STATIC_CONFIG.dynamic_config_path)?;
    Ok(CONFIG_DIRECTORY
        .get_or_init(|| PathBuf::from(&*path))
        .as_path())
}

fn get_config_file() -> Result<&'static Path> {
    if let Some(path) = CONFIG_FILE.get() {
        return Ok(path.as_path());
    }

    let path = get_config_directory()?.join("config.toml");
    Ok(CONFIG_FILE.get_or_init(|| path).as_path())
}

pub fn get_static() -> &'static StaticConfig {
    &STATIC_CONFIG
}

pub fn get() -> Result<MutexGuard<'static, DynamicConfig>> {
    if let Some(mutex) = CONFIG.get() {
        return mutex
            .lock()
            .map_err(|_| Error::GlobalMutexPoisoned(GlobalMutex::Config));
    }

    let config_dir = get_config_directory()?;
    let config_file = get_config_file()?;

    let config: DynamicConfig = if config_file.exists() {
        let toml_string = fs::read_to_string(config_file)?;
        toml::from_str(&toml_string)?
    } else {
        fs::create_dir_all(config_dir)?;
        let config = get_default_dynamic_config();
        fs::write(config_file, toml::to_string(config)?)?;
        config.clone()
    };

    CONFIG.initial_value.get_or_init(|| config.clone());

    CONFIG
        .get_or_init(|| Mutex::new(config))
        .lock()
        .map_err(|_| Error::GlobalMutexPoisoned(GlobalMutex::Config))
}

pub fn get_expanded_servers_dir() -> Result<&'static Path> {
    if let Some(dir) = EXPANDED_SERVERS_DIR.get() {
        return Ok(dir.as_path());
    }

    let config = get()?;
    let dir = shellexpand::full(&config.servers_directory)?;
    Ok(EXPANDED_SERVERS_DIR
        .get_or_init(|| PathBuf::from(&*dir))
        .as_path())
}

pub fn get_current_server_directory() -> Result<String> {
    let servers_dir = get_expanded_servers_dir()?;
    let current_dir = env::current_dir()?;

    if !current_dir.starts_with(servers_dir) {
        return Err(Error::InvalidServersDirectory);
    }

    let server = current_dir
        .strip_prefix(servers_dir)?
        .components()
        .next()
        .ok_or(Error::NoServerChild)?
        .as_os_str()
        .to_string_lossy()
        .to_string();

    Ok(server)
}

#[macro_export]
macro_rules! handle_server_arg {
    ($server:expr) => {
        (|| {
            use $crate::config::{get, get_current_server_directory};

            let server = $server
                .map_or_else::<Result<String>, _, _>(
                    || Ok(get()?.default_server.clone()),
                    |val| Ok(val),
                )
                .wrap_err("Failed to get configuration")?;

            if server == "." {
                get_current_server_directory().wrap_err("Failed to get current server directory")
            } else {
                Ok(server)
            }
        })()
    };
}
