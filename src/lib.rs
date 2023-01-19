#![allow(incomplete_features)]
// opt-out of using the unstable feature "async_fn_in_trait". See https://github.com/rust-lang/rust/issues/91611.
#![cfg_attr(feature = "enable_afit", feature(async_fn_in_trait))]
mod util;
pub mod advertisement;
pub mod session;
pub use bluer;