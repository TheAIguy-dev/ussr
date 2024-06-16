//! All supported versions of the protocol.
//!
//! This module exports everything from the latest enabled version.

#[cfg(feature = "v1_7_2")]
pub mod v1_7_2;

cfg_if::cfg_if! {
    if #[cfg(feature = "v1_7_2")] {
        pub use v1_7_2::*;
    }
}
