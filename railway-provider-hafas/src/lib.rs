#![doc = include_str!("../README.md")]

pub mod api;
pub mod client;
pub mod error;
pub mod format;
pub mod parse;
pub mod profile;
// TODO
// mod serialize;

pub use error::{Error, ParseError, ParseResult, Result};
pub use profile::Profile;
use rcore::*;
