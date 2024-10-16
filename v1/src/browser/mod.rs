// src/browser/mod.rs

pub mod interface;
pub mod tabs;

pub use interface::start;
pub use tabs::{Browser, Tab};
