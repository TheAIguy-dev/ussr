//! Convenience module for exporting all protocol versions and re-exporting everything from the latest enabled version.

#[cfg(feature = "v1_7_2")]
#[path = "v1_7_2/mod.rs"]
pub mod v1_7_2;

cfg_if::cfg_if! {
    if #[cfg(feature = "v1_7_2")] {
        pub use v1_7_2::*;
    }
}
