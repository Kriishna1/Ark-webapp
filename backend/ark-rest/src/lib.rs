#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_repr;
extern crate url;

mod client;

// TODO: Should not need to expose these modules once every method is implemented in `client`.
pub mod apis;
pub mod models;

pub use client::Client;
pub use client::Error;
