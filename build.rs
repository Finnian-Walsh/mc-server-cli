use color_eyre::eyre::{Result, WrapErr};
use config::{DynamicConfig, StaticConfig};
use quote::quote;
use serde::Deserialize;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize)]
struct Config {
    static_config: StaticConfig<String>,
    default_dynamic_config: DynamicConfig,
}

macro_rules! build_log {
    ($($arg:tt)*) => {
        #[cfg(feature = "build-logging")]
        println!("cargo:warning={}", format!($($arg)*))
    }
}

fn main() -> Result<()> {
    build_log!("Build script running...");
    color_eyre::install()?;
    println!("cargo:rerun-if-changed=");

    let cargo_manifest_dir = PathBuf::new().join(env::var("CARGO_MANIFEST_DIR")?);
    build_log!("Manifest directory: {cargo_manifest_dir:?}");

    let cfg_generation_file = &cargo_manifest_dir
        .join("config")
        .join("src")
        .join("generated_cfg.rs");

    let config_template_path = cargo_manifest_dir.join("config_template.toml");

    if config_template_path.exists() {
        build_log!("Configuration path exists ({config_template_path:?})");

        if !config_template_path.is_file() {
            build_log!(
                "Static configuration ({}) should be a file - the default static configuration will be used",
                config_template_path
                    .components()
                    .next_back()
                    .map(|c| format!("{:?}", c))
                    .as_deref()
                    .unwrap_or("unknown")
            );
            return Ok(());
        }

        let config: Config = toml::from_str(
            &fs::read_to_string(config_template_path)
                .wrap_err("Failed to read configuration file")?,
        )
        .wrap_err("Failed to parse configuration file")?;

        let static_config = config.static_config;
        let default_dynamic_config = config.default_dynamic_config;

        let tokens = quote! {
            pub const STATIC_CONFIG: StaticConfig = #static_config;
            #default_dynamic_config
        };

        fs::write(cfg_generation_file, tokens.to_string())?;

        let expanded_dynamic_config_dir = shellexpand::full(&static_config.dynamic_config_path)?;
        let dynamic_config_template_path =
            Path::new(&*expanded_dynamic_config_dir).join("config.toml");

        if dynamic_config_template_path.exists() {
            if !dynamic_config_template_path.is_file() {
                build_log!(
                    "There is something at the path where the dynamic configuration is supposed to exist; this will cause problems in the future"
                );
            } else {
                build_log!("Dynamic configuration found");
            }
        } else {
            fs::create_dir_all(&*expanded_dynamic_config_dir)?;
            fs::write(
                &dynamic_config_template_path,
                toml::to_string(&default_dynamic_config)
                    .wrap_err("Failed to serialize dynamic configuration")?,
            )
            .wrap_err_with(|| {
                format!(
                    "Failed to write to the dynamic configuration path ({:?})",
                    &dynamic_config_template_path
                )
            })?;
        }

        build_log!("Configuration has been generated");
    } else {
        build_log!("Config path ({config_template_path:?}) does not exist");
    }

    Ok(())
}
