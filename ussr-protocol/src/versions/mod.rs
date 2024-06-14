//! All supported versions of the protocol.
//!
//! Every enabled version will be imported, but only the latest will be exported.

mod common;

#[cfg(feature = "v1_7_2")]
pub mod v1_7_2;

cfg_if::cfg_if! {
    if #[cfg(feature = "v1_7_2")] {
        pub use v1_7_2::*;
    }
}
