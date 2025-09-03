use config::{DynamicConfig, StaticConfig};
use std::{
    env, fs, io,
    path::PathBuf,
};
use thiserror::Error;
use toml;
use quote::quote;
use serde::Deserialize;

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Toml(#[from] toml::de::Error),

    #[error(transparent)]
    Var(#[from] env::VarError),
}

#[derive(Debug, Deserialize)]
struct Config {
    static_config: StaticConfig<String>,
    default_dynamic_config: DynamicConfig<String>,
}

fn main() -> Result<(), Error> {
    let cargo_manifest_dir = PathBuf::new().join(env::var("CARGO_MANIFEST_DIR")?);
    let cfg_generation_file = &cargo_manifest_dir.join("config").join("src").join("generated_cfg.rs");
    println!("cargo:rerun-if-changed={}", cfg_generation_file.display());

    let config_path = cargo_manifest_dir.join("config.toml");
    println!("cargo:rerun-if-changed={}", config_path.display());

    if config_path.exists() {
        if !config_path.is_file() {
            println!(
                "cargo:warning=static configuration ({}) should be a file - the default static configuration will be used",
                config_path.components().last().map(|c| format!("{:?}", c)).unwrap_or_else(|| String::from("unknown")));
            return Ok(());
        }

        let config: Config = toml::from_str(&fs::read_to_string(config_path)?)?;
        let static_config = config.static_config;
        let default_dynamic_config = config.default_dynamic_config;

        let tokens = quote! {
            use super::{StaticConfig, DynamicConfig};
            pub const STATIC_CONFIG: StaticConfig = #static_config;
            pub const DEFAULT_DYNAMIC_CONFIG: DynamicConfig = #default_dynamic_config;
        };

        fs::write(cfg_generation_file, tokens.to_string())?;

        println!("cargo:rustc-cfg=config_generated");
    }

    Ok(())
}
