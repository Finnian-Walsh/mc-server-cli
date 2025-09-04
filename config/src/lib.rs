use cfg_if::cfg_if;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use serde::{Deserialize, Serialize};
use shellexpand;
use std::env;

cfg_if! {
    if #[cfg(config_generated)] {
        mod generated_cfg;

        pub use generated_cfg::STATIC_CONFIG;
        pub use generated_cfg::DEFAULT_DYNAMIC_CONFIG;
    } else {
        pub const STATIC_CONFIG: StaticConfig = StaticConfig {
            contact: "none",
            dynamic_config_path: "~/.config/mc-server",
        };

        pub const DEFAULT_DYNAMIC_CONFIG: DynamicConfig = DynamicConfig {
            default_java_args: "",
            servers_directory: "~/Servers",
            default_server: "",
        };
    }
}

pub trait AllowedConfigValue {}
impl AllowedConfigValue for String {}
impl AllowedConfigValue for &'static str {}

#[derive(Debug, Deserialize)]
pub struct StaticConfig<T: AllowedConfigValue = &'static str> {
    pub contact: T,
    pub dynamic_config_path: T,
}

impl ToTokens for StaticConfig<String> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let contact = &self.contact;
        let dynamic_config_path = &self.dynamic_config_path;
        tokens.extend(quote! {
            StaticConfig {
                contact: #contact,
                dynamic_config_path: #dynamic_config_path,
            }
        });
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DynamicConfig<T: AllowedConfigValue = &'static str> {
    pub default_java_args: T,
    pub servers_directory: T,
    pub default_server: T,
}

impl ToTokens for DynamicConfig<String> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let default_java_args = &self.default_java_args;
        let servers_directory = &self.servers_directory;
        let default_server = &self.default_server;

        tokens.extend(quote! {
            DynamicConfig {
                default_java_args: #default_java_args,
                servers_directory: #servers_directory,
                default_server: #default_server,
            }
        });
    }
}

impl<E> From<&DynamicConfig<&'static str>> for Result<DynamicConfig<String>, E>
where
    E: From<shellexpand::LookupError<env::VarError>>,
{
    fn from(config: &DynamicConfig<&'static str>) -> Self {
        Ok(DynamicConfig::<String> {
            default_java_args: config.default_java_args.to_string(),
            servers_directory: config.servers_directory.to_string(),
            default_server: config.default_server.to_string(),
        })
    }
}

