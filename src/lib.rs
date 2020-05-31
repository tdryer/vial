#![allow(unused)]

#[macro_use]
mod macros;
pub mod asset;
mod bundler;
pub mod features;
mod http_request;
mod method;
pub mod prelude;
mod request;
mod responder;
mod response;
mod router;
mod server;
mod state;
mod util;

#[cfg(features = "horror")]
#[macro_use]
extern crate horrorshow;

#[cfg(features = "horror")]
pub use features::horrorshow::{box_html, html, owned_html};

pub use {
    bundler::bundle_assets, http_request::HttpRequest, method::Method, request::Request,
    responder::Responder, response::Response, router::Router, server::run, state::State,
};
pub type Result<T> = std::result::Result<T, std::io::Error>;

/// Directory where assets are stored, if any.
pub static mut ASSET_DIR: Option<&'static str> = None;

/// Assets bundled into the binary in release mode.
pub static mut BUNDLED_ASSETS: Option<std::collections::HashMap<String, &'static [u8]>> = None;

/// Date and time this program was compiled.
pub const BUILD_DATE: &str = env!("BUILD_DATE");
