#[cfg(not(disable_tokenization))]
use proc_macro2::TokenStream;

#[cfg(not(disable_tokenization))]
use quote::{ToTokens, quote};

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    ffi::OsStr,
    fmt::{self, Debug, Formatter},
};

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

#[cfg(not(disable_tokenization))]
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

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct Password(pub String);

impl AsRef<OsStr> for Password {
    fn as_ref(&self) -> &OsStr {
        OsStr::new(&self.0)
    }
}

#[cfg(not(disable_tokenization))]
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RconConfig {
    pub server_address: Option<String>,
    pub port: Option<u16>,
    pub password: Option<Password>,
}

#[cfg(not(disable_tokenization))]
impl ToTokens for RconConfig {
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
            Some(password) => quote! { Some(Password(String::from(#password))) },
            None => quote! { None },
        };

        tokens.extend(quote! {
            RconConfig {
                server_address: #server_address,
                port: #port,
                password: #password,
            }
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DynamicConfig {
    pub default_java_args: String,
    pub nogui: bool,
    pub servers_directory: String,
    pub default_server: String,
    pub rcon: HashMap<String, RconConfig>,
}

#[cfg(not(disable_tokenization))]
impl ToTokens for DynamicConfig {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let default_java_args = &self.default_java_args;
        let nogui = &self.nogui;
        let servers_directory = &self.servers_directory;
        let default_server = &self.default_server;

        let key_value_pairs = self.rcon.iter().map(|(k, v)| {
            quote! { ( String::from(#k), #v )}
        });

        tokens.extend(quote! {
            DynamicConfig {
                default_java_args: String::from(#default_java_args),
                nogui: #nogui,
                servers_directory: String::from(#servers_directory),
                default_server: String::from(#default_server),
                rcon: std::collections::HashMap::from([
                    #(#key_value_pairs),*
                ]),
            }
        });
    }
}
