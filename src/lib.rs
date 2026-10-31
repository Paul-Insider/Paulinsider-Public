//! Focused Rust examples for multilingual websites and media delivery.
//!
//! This crate contains pure functions, no network client and no web server.
//! Applications remain responsible for trusted request metadata, authorization,
//! cache headers, HTTP response policy and actual media streaming.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod language;
pub mod media_range;
