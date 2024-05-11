#[macro_use]
extern crate log;
#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate serde;

pub mod api;
pub mod constant;
pub mod errors;
mod frb_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
pub mod repository;
pub mod domain;
pub mod service;
#[cfg(test)]
mod tests;
