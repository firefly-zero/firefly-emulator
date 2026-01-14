#![deny(clippy::pedantic)]
#![allow(
    clippy::wildcard_imports,
    clippy::missing_errors_doc,
    clippy::many_single_char_names,
    clippy::enum_glob_use,
    clippy::match_same_arms
)]

mod cli_args;
mod core;
mod display;
mod error;

pub use cli_args::CliArgs;
pub use core::run_emulator;
pub use error::Error;
