//! # weaviate-community
//!
//! Community client for handling Weaviate vector database transactions written in Rust, for Rust.
//! More information on Weaviate can be found on the official Weaviate webpage.
mod client;
mod endpoints;
pub use client::{WeaviateClient, WeaviateClientBuilder};

pub mod models;
