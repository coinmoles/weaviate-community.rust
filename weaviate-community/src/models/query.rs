//! GraphQL related structures.
//!
//! Contains the ability to generate the following queries:
//! - Get
//! - Aggregate
//! - Explore
//!
//! Also contains the ability to create a raw query from a string.
//!
//! This is currently incomplete - most of the data types for the different options are Strings,
//! which I want to enforce to the expected values a little better. Mainly just getting the
//! structure and ability to run completed now.
//!
//! There are also some places I need to return an error from which I am yet to do.
//!
//! I've also not had a chance to test a lot of the functionality, so lots will be broken like the
//! near_text or near_image as I have not implemented the `encoding` functionality yet.

mod raw;
pub use raw::*;

mod aggregate;
pub use aggregate::*;

mod explore;
pub use explore::*;

mod get;
pub use get::*;
