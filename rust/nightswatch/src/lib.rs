pub mod daemon;
mod platform;

#[cfg(feature = "cli")]
pub use platform::cli;
