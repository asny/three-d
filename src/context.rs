//!
//! Low-level graphics abstraction layer which maps one-to-one with the OpenGL graphics API on native
//! and WebGL2 graphics API on web. This is just a re-export of the [glow](https://crates.io/crates/glow) crate.
//! Use this if you want to have complete control of a feature but be aware that there are no safety checks.
//!

pub use glow::*;
