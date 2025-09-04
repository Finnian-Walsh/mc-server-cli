use crate::{
    error::{Error, Mutexes, Result},
    home,
};
use config::{DEFAULT_DYNAMIC_CONFIG, DynamicConfig, STATIC_CONFIG};
use std::{
    fs,
    ops::Deref,
    path::{Path, PathBuf},
    sync::{Mutex, MutexGuard, OnceLock},
};
use toml;

struct AutoConfig {
    // type ValueType = OnceLock<Mutex<DynamicConfig<String>>>;
    value: <Self as Deref>::Target,
}

impl AutoConfig {
    fn write(&self) -> Result<()> {
        let Some(mutex) = self.get() else {
            return Ok(());
        };

        fs::create_dir_all(get_config_directory()?)?;
        let guard = mutex.lock().map_err(|_| Error::Poison(Mutexes::Config))?;
        fs::write(get_config_file()?, toml::to_string(&*guard)?)?;
        Ok(())
    }
}

impl Deref for AutoConfig {
    type Target = OnceLock<Mutex<DynamicConfig<String>>>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Drop for AutoConfig {
    fn drop(&mut self) {
        if let Err(err) = self.write() {
            eprintln!("Failed to write config: {}", err);
            return;
        }
    }
}

static CONFIG: AutoConfig = AutoConfig {
    value: OnceLock::new(),
};

static CONFIG_DIRECTORY: OnceLock<PathBuf> = OnceLock::new();
static CONFIG_FILE: OnceLock<PathBuf> = OnceLock::new();

fn get_config_directory() -> Result<&'static Path> {
    if let Some(path) = CONFIG_DIRECTORY.get() {
        return Ok(path.as_path());
    }

    let path = home::get()?.join(STATIC_CONFIG.dynamic_config_path);
    Ok(CONFIG_DIRECTORY.get_or_init(|| path).as_path())
}

fn get_config_file() -> Result<&'static Path> {
    if let Some(path) = CONFIG_FILE.get() {
        return Ok(path.as_path());
    }

    let path = get_config_directory()?.join("config.toml");
    Ok(CONFIG_FILE.get_or_init(|| path).as_path())
}

pub fn get() -> Result<MutexGuard<'static, DynamicConfig<String>>> {
    if let Some(mutex) = CONFIG.get() {
        return mutex.lock().map_err(|_| Error::Poison(Mutexes::Config));
    }

    let config_dir = get_config_directory()?;
    let config_file = get_config_file()?;

    let config: DynamicConfig<String> = if config_file.exists() {
        let toml_string = fs::read_to_string(config_file)?;
        toml::from_str(&toml_string)?
    } else {
        fs::create_dir_all(config_dir)?;
        fs::write(config_file, toml::to_string(&DEFAULT_DYNAMIC_CONFIG)?)?;
        let result: Result<DynamicConfig<String>> = (&DEFAULT_DYNAMIC_CONFIG).into();
        result?
    };
    
    CONFIG
        .get_or_init(|| Mutex::new(config))
        .lock()
        .map_err(|_| Error::Poison(Mutexes::Config))
}

pub fn unwrap_or_default<'a>(server: Option<String>) -> Result<String> {
    server.map_or_else(|| Ok(get()?.default_server.clone()), |val| Ok(val))
}
