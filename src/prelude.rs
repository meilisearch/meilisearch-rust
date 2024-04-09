#[cfg(feature = "isahc")]
#[cfg(not(target_arch = "wasm32"))]
pub type Client = crate::client::Client<crate::request::IsahcClient>;
#[cfg(feature = "isahc")]
#[cfg(not(target_arch = "wasm32"))]
pub type Index = crate::indexes::Index<crate::request::IsahcClient>;

#[cfg(target_arch = "wasm32")]
pub type Client = crate::client::Client<crate::request::WebSysClient>;
#[cfg(target_arch = "wasm32")]
pub type Index = crate::indexes::Index<crate::request::WebSysClient>;

pub type Settings = crate::settings::Settings;
