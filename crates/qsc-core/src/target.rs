#![allow(unexpected_cfgs)]

#[cfg(target_env = "gnu")]
pub const ENV: &str = "gnu";

#[cfg(target_env = "gnueabi")]
pub const ENV: &str = "gnueabi";

#[cfg(target_env = "gnueabihf")]
pub const ENV: &str = "gnueabihf";

#[cfg(target_env = "musl")]
pub const ENV: &str = "musl";

#[cfg(target_env = "musleabi")]
pub const ENV: &str = "musleabi";

#[cfg(target_env = "musleabihf")]
pub const ENV: &str = "musleabihf";

#[cfg(target_os = "android")]
pub const ENV: &str = "android";
