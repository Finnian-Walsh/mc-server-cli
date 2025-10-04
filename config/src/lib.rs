use cfg_if::cfg_if;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug, sync::OnceLock};

cfg_if! {
    if #[cfg(config_generated)] {
        mod generated_cfg;

        pub use generated_cfg::STATIC_CONFIG;
        pub use generated_cfg::get_default_dynamic_config;
    } else {
        pub const STATIC_CONFIG: StaticConfig = StaticConfig {
            contact: "none",
            dynamic_config_path: "~/.config/mcserver",
        };

        static DEFAULT_DYNAMIC_CONFIG: OnceLock<DynamicConfig> = OnceLock::new();

        pub fn get_default_dynamic_config() -> &'static DynamicConfig {
            DEFAULT_DYNAMIC_CONFIG.get_or_init(|| {
                DynamicConfig {
                    default_java_args: String::from(""),
                    servers_directory: String::from("~/Servers"),
                    default_server: String::from(""),
                    mcrcon: None,
                }
            })
        }
    }
}

pub trait AllowedConfigValue {}
impl AllowedConfigValue for String {}
impl AllowedConfigValue for &'static str {}

#[derive(Debug, Deserialize)]
pub struct StaticConfig<T = &'static str>
where
    T: AllowedConfigValue,
{
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct McrconConfig {
    pub server_address: Option<String>,
    pub port: Option<u16>,
    pub password: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DynamicConfig {
    pub default_java_args: String,
    pub servers_directory: String,
    pub default_server: String,
    pub mcrcon: Option<HashMap<String, McrconConfig>>,
}

impl ToTokens for DynamicConfig {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let default_java_args = &self.default_java_args;
        let servers_directory = &self.servers_directory;
        let default_server = &self.default_server;

        tokens.extend(quote! {
            pub static DEFAULT_DYNAMIC_CONFIG: OnceLock<DynamicConfig> = OnceLock::new();

            pub fn get_default_dynamic_config() -> &'static DynamicConfig {
                DEFAULT_DYNAMIC_CONFIG.get_or_init(|| )
                default_java_args: String::from(#default_java_args),
                servers_directory: String::from(#servers_directory),
                default_server: String::from(#default_server),
                mcrcon: None,
            }
        });
    }
}
