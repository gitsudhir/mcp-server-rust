//! Model Context Protocol (MCP) Server Implementation in Rust
//!
//! This library provides a framework for building MCP servers that expose
//! tools, resources, and prompts to LLM applications.

pub mod server;
pub mod tools;
pub mod resources;
pub mod prompts;
pub mod utils;
pub mod transport;

pub use server::{McpServer, ServerConfig};
pub use utils::error::{Error, Result};

pub const PROTOCOL_VERSION: &str = "2024-11-05";