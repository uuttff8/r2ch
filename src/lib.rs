//! # R2ch
//! 
//! The `R2ch` crate provides a simple twoch implementation
//! 
//! - Uses reqwest crate for methods
//! - Json
//! 
//! ## How to use
//! 
//! for get all boards, you can use boards_all() method.
//! 
//! ```rust
//! use r2ch::client::TwoCH;
//! 
//! let _ = TwoCH::default().boards_all();
//! ```


pub mod client;
mod json;
