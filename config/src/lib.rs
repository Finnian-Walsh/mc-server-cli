use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    ffi::OsStr,
    fmt::{self, Debug, Formatter},
    sync::OnceLock,
};

#[cfg(config_generated)]
mod generated_cfg;

#[cfg(config_generated)]
pub use generated_cfg::STATIC_CONFIG;

#[cfg(config_generated)]
pub use generated_cfg::get_default_dynamic_config;

#[cfg(not(config_generated))]
pub const STATIC_CONFIG: StaticConfig = StaticConfig {
    contact: "none",
    dynamic_config_path: "~/.config/mcserver",
};

#[cfg(not(config_generated))]
static DEFAULT_DYNAMIC_CONFIG: OnceLock<DynamicConfig> = OnceLock::new();

#[cfg(not(config_generated))]
pub fn get_default_dynamic_config() -> &'static DynamicConfig {
    DEFAULT_DYNAMIC_CONFIG.get_or_init(|| DynamicConfig {
        default_java_args: String::from(""),
        nogui: false,
        servers_directory: String::from("~/Servers"),
        default_server: String::from(""),
        mcrcon: HashMap::new(),
    })
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

#[derive(Clone, Deserialize, Serialize)]
pub struct Password(String);

impl AsRef<OsStr> for Password {
    fn as_ref(&self) -> &OsStr {
        OsStr::new(&self.0)
    }
}

impl ToTokens for Password {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let password = &self.0;
        tokens.extend(quote! {#password})
    }
}

impl Debug for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(hidden)")
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct McrconConfig {
    pub server_address: Option<String>,
    pub port: Option<u16>,
    pub password: Option<Password>,
}

impl ToTokens for McrconConfig {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let server_address = match self.server_address.as_ref() {
            Some(server_address) => quote! {
                Some(String::from(#server_address))
            },
            None => quote! { None },
        };

        let port = match self.port {
            Some(port) => quote! { Some(#port) },
            None => quote! { None },
        };

        let password = match self.password.as_ref() {
            Some(password) => quote! { Some(String::from(#password)) },
            None => quote! { None },
        };

        tokens.extend(quote! {
            McrconConfig {
                server_address: #server_address,
                port: #port,
                password: String::from(#password),
            }
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DynamicConfig {
    pub default_java_args: String,
    pub nogui: bool,
    pub servers_directory: String,
    pub default_server: String,
    pub mcrcon: HashMap<String, McrconConfig>,
}

impl ToTokens for DynamicConfig {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let default_java_args = &self.default_java_args;
        let nogui = &self.nogui;
        let servers_directory = &self.servers_directory;
        let default_server = &self.default_server;

        let key_value_pairs = self.mcrcon.iter().map(|(k, v)| {
            quote! { (#k, #v )}
        });

        tokens.extend(quote! {
            use std::collections::HashMap;

            pub static DEFAULT_DYNAMIC_CONFIG: OnceLock<DynamicConfig> = OnceLock::new();

            pub fn get_default_dynamic_config() -> &'static DynamicConfig {
                DEFAULT_DYNAMIC_CONFIG.get_or_init(||
                    DynamicConfig {
                        default_java_args: String::from(#default_java_args),
                        nogui: #nogui,
                        servers_directory: String::from(#servers_directory),
                        default_server: String::from(#default_server),
                        mcrcon: HashMap::from([
                            #(#key_value_pairs),*
                        ]),
                    }
                )
            }
        });
    }
}
