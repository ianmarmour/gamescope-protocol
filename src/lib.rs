
//! This crate provides bindings to the official wayland protocol extensions
//! provided in <https://github.com/Plagman/gamescope/tree/master/protocol>
//!
//! These bindings are built on top of the crates wayland-client and wayland-server.
//!
//! Each protocol module contains a `client` and a `server` submodules, for each side of the
//! protocol. The creation of these modules (and the dependency on the associated crate) is
//! controlled by the two cargo features `client` and `server`.
//!

#![warn(missing_docs)]

#[macro_use]
mod protocol_macro;

mod stable;
pub use stable::*;