use color_eyre::eyre::{Result, WrapErr};
use config::{DynamicConfig, StaticConfig};
use quote::quote;
use serde::Deserialize;
use shellexpand;
use std::{
    env, fs,
    path::{Path, PathBuf},
};
use toml;

#[derive(Debug, Deserialize)]
struct Config {
    static_config: StaticConfig<String>,
    default_dynamic_config: DynamicConfig<String>,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cargo_manifest_dir = PathBuf::new().join(env::var("CARGO_MANIFEST_DIR")?);
    let cfg_generation_file = &cargo_manifest_dir
        .join("config")
        .join("src")
        .join("generated_cfg.rs");
    println!("cargo:rerun-if-changed={}", cfg_generation_file.display());

    let config_path = cargo_manifest_dir.join("config.toml");
    println!("cargo:rerun-if-changed={}", config_path.display());

    if config_path.exists() {
        if !config_path.is_file() {
            println!(
                "cargo:warning=static configuration ({}) should be a file - the default static configuration will be used",
                config_path
                    .components()
                    .last()
                    .map(|c| format!("{:?}", c))
                    .unwrap_or_else(|| String::from("unknown"))
            );
            return Ok(());
        }

        let config: Config = toml::from_str(
            &fs::read_to_string(config_path).wrap_err("Failed to read configuration file")?,
        )
        .wrap_err("Failed to parse configuration file")?;

        let static_config = config.static_config;
        let default_dynamic_config = config.default_dynamic_config;

        let tokens = quote! {
            use super::{StaticConfig, DynamicConfig};
            pub const STATIC_CONFIG: StaticConfig = #static_config;
            pub const DEFAULT_DYNAMIC_CONFIG: DynamicConfig = #default_dynamic_config;
        };

        fs::write(cfg_generation_file, tokens.to_string())?;

        let expanded_dynamic_config_dir = shellexpand::full(&static_config.dynamic_config_path)?;
        let dynamic_config_path = Path::new(&*expanded_dynamic_config_dir).join("config.toml");

        if dynamic_config_path.exists() {
            if !dynamic_config_path.is_file() {
                println!(
                    "cargo:warning=There is something at the path where the dynamic configuration is supposed to exist; this will cause problems in the future"
                );
            }

            return Ok(());
        } else {
            fs::write(
                &dynamic_config_path,
                toml::to_string(&default_dynamic_config)
                    .wrap_err("Failed to serialize dynamic configuration")?,
            )
            .wrap_err_with(|| {
                format!(
                    "Failed to write to the dynamic configuration path ({:?})",
                    &dynamic_config_path
                )
            })?;
        }

        println!("cargo:rustc-cfg=config_generated");
    }

    Ok(())
}
