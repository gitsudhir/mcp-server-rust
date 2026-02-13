# Rust MCP Server: A Complete Implementation

A full-featured Model Context Protocol (MCP) server implementation in Rust, providing tools, resources, and prompts for LLM integration.

## Table of Contents

1. [Overview](#overview)
2. [Features](#features)
3. [Project Structure](#project-structure)
4. [Prerequisites](#prerequisites)
5. [Installation](#installation)
6. [Building the Project](#building-the-project)
7. [Running the Server](#running-the-server)
8. [Testing the Server](#testing-the-server)
9. [API Reference](#api-reference)
10. [Adding Custom Tools](#adding-custom-tools)
11. [Troubleshooting](#troubleshooting)
12. [Security Considerations](#security-considerations)

---

## Overview

This Rust MCP Server implements the **Model Context Protocol (MCP)** specification, enabling seamless integration between LLM applications and external tools, resources, and knowledge bases.

### What is MCP?

The Model Context Protocol is an open standard that allows:
- **LLM Applications** to connect to external data sources and tools
- **Servers** to expose capabilities (tools, resources, prompts) in a standardized way
- **Secure Integration** without modifying client applications

### Key Benefits

✅ **Type-Safe**: Leverages Rust's type system for safety  
✅ **Performant**: Async/await with Tokio for high concurrency  
✅ **Modular**: Easy to add custom tools and resources  
✅ **Secure**: Built-in validation and error handling  
✅ **Stdio-Based**: Simple deployment as a subprocess  

---

## Features

### Built-in Tools

1. **Greeting Tool** (`greet`)
   - Greets users with personalized messages
   - Input: `name` (string)
   - Output: Greeting message

2. **BMI Calculator Tool** (`calculate-bmi`)
   - Calculates Body Mass Index
   - Inputs: `weightKg` (number), `heightM` (number)
   - Output: Calculated BMI value

3. **Weather Tool** (`fetch-weather`)
   - Fetches weather information (simulated)
   - Input: `city` (string)
   - Output: Weather data (temperature, condition, humidity, etc.)

### Built-in Resources

1. **Application Configuration** (`config://app`)
   - Provides application metadata and settings
   - Returns JSON configuration

### Built-in Prompts

1. **Code Review** (`review-code`)
   - Generates prompts for LLM to review code
   - Arguments: `code` (required), `focus` (optional: performance, security, style, general)

### Protocol Support

- ✅ JSON-RPC 2.0 compliant
- ✅ Stdio transport (newline-delimited JSON)
- ✅ Proper error handling with standard error codes
- ✅ Logging to stderr
- ✅ Protocol versioning (2024-11-05)

---

## Project Structure

```
mcp-server-rust/
├── Cargo.toml                          # Project manifest
├── README.md                           # This file
├── src/
│   ├── main.rs                         # Entry point
│   ├── lib.rs                          # Library exports
│   ├── server.rs                       # MCP server implementation
│   ├── tools/
│   │   ├── mod.rs                      # Tool definitions
│   │   ├── greeting_tool.rs            # Greeting tool implementation
│   │   ├── calculator_tool.rs          # BMI calculator tool
│   │   └── weather_tool.rs             # Weather tool (simulated)
│   ├── resources/
│   │   ├── mod.rs                      # Resource definitions
│   │   ├── config_resource.rs          # App config resource
│   │   └── file_resource.rs            # File-based resource
│   ├── prompts/
│   │   ├── mod.rs                      # Prompt definitions
│   │   └── code_review_prompt.rs       # Code review prompt
│   ├── transport/
│   │   ├── mod.rs                      # Transport trait
│   │   └── stdio.rs                    # Stdio implementation
│   └── utils/
│       ├── mod.rs                      # Utility modules
│       ├── logger.rs                   # Logging utilities
│       └── error.rs                    # Error types
└── data/
    └── (sample data files)
```

---

## Prerequisites

### System Requirements

- **OS**: macOS, Linux, or Windows (with WSL2)
- **RAM**: 512 MB minimum (2 GB recommended)
- **Disk Space**: 1 GB (mostly for dependencies)

### Required Software

1. **Rust** (1.70 or later)
   ```bash
   # Install from https://rustup.rs/
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Cargo** (comes with Rust)
   ```bash
   cargo --version
   ```

3. **Git**
   ```bash
   git --version
   ```

### Verify Installation

```bash
rustc --version
cargo --version
```

---

## Installation

### Step 1: Clone the Repository

```bash
git clone https://github.com/gitsudhir/mcp-server-rust.git
cd mcp-server-rust
```

Or create a new project from scratch:

```bash
cargo new mcp-server-rust
cd mcp-server-rust
```

### Step 2: Copy Files

Copy all provided source files into the `src/` directory as shown in the Project Structure section.

### Step 3: Update Cargo.toml

Ensure your `Cargo.toml` matches the provided configuration with all required dependencies.

### Step 4: Verify Dependencies

```bash
cargo check
```

This downloads and verifies all dependencies without building.

---

## Building the Project

### Development Build

```bash
# Build with debug information (slower, larger binary)
cargo build

# Output: target/debug/mcp-server-rust
```

### Release Build (Recommended)

```bash
# Build optimized binary (faster execution)
cargo build --release

# Output: target/release/mcp-server-rust
```

### Build Verification

```bash
# Check without building
cargo check

# Run tests
cargo test

# View warnings
cargo clippy
```

---

## Running the Server

### Basic Execution

```bash
# Using debug binary
./target/debug/mcp-server-rust

# Using release binary
./target/release/mcp-server-rust
```

### With Logging

```bash
# Enable debug logging
RUST_LOG=debug ./target/release/mcp-server-rust

# Enable specific module logging
RUST_LOG=rust_mcp_server::server=debug,rust_mcp_server::transport=debug ./target/release/mcp-server-rust

# All logs
RUST_LOG=trace ./target/release/mcp-server-rust
```

### Creating a Shell Wrapper

For easier invocation, create a shell script:

```bash name=mcp-server.sh
#!/bin/bash
exec /full/path/to/mcp-server-rust "$@"
```

Make it executable:

```bash
chmod +x mcp-server.sh
```

---

## Testing the Server

### Manual Testing with JSON-RPC

Test in another terminal while the server is running:

#### Initialize Connection

```bash
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"TestClient","version":"1.0.0"}}}' | ./target/release/mcp-server-rust
```

#### Call a Tool

```bash
echo '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"greet","arguments":{"name":"Alice"}}}' | ./target/release/mcp-server-rust
```

#### List Tools

```bash
echo '{"jsonrpc":"2.0","id":3,"method":"tools/list"}' | ./target/release/mcp-server-rust
```

#### Calculate BMI

```bash
echo '{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"calculate-bmi","arguments":{"weightKg":70,"heightM":1.75}}}' | ./target/release/mcp-server-rust
```

#### Fetch Weather

```bash
echo '{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"fetch-weather","arguments":{"city":"San Francisco"}}}' | ./target/release/mcp-server-rust
```

#### Get Prompt

```bash
echo '{"jsonrpc":"2.0","id":6,"method":"prompts/get","params":{"name":"review-code","arguments":{"code":"fn main() { println!(\"Hello!\"); }","focus":"performance"}}}' | ./target/release/mcp-server-rust
```

### Testing with MCP Inspector

The official MCP Inspector tool allows interactive testing:

1. **Install Inspector**:
   ```bash
   npm install -g @modelcontextprotocol/inspector
   ```

2. **Run Server**:
   ```bash
   ./target/release/mcp-server-rust
   ```

3. **Run Inspector** (in another terminal):
   ```bash
   mcp-inspector stdio ./target/release/mcp-server-rust
   ```

4. Open the Inspector UI in your browser and test tools interactively.

### Automated Testing

```bash
# Run built-in tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name -- --nocapture
```

---

## API Reference

### JSON-RPC Methods

#### 1. `initialize`

**Purpose**: Initialize the MCP connection

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "protocolVersion": "2024-11-05",
    "capabilities": {},
    "clientInfo": {
      "name": "ClientName",
      "version": "1.0.0"
    }
  }
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "protocolVersion": "2024-11-05",
    "capabilities": {
      "tools": {},
      "resources": {},
      "prompts": {}
    },
    "serverInfo": {
      "name": "RustMcpServer",
      "version": "1.0.0"
    }
  }
}
```

---

#### 2. `tools/list`

**Purpose**: List available tools

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/list"
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "tools": [
      {
        "name": "greet",
        "description": "Greets a person with a friendly message",
        "inputSchema": {
          "type": "object",
          "properties": {
            "name": {
              "type": "string",
              "description": "The name of the person to greet"
            }
          },
          "required": ["name"]
        },
        "annotations": {
          "title": "Greet Tool",
          "readOnlyHint": true
        }
      }
    ]
  }
}
```

---

#### 3. `tools/call`

**Purpose**: Call a tool

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "greet",
    "arguments": {
      "name": "Alice"
    }
  }
}
```

**Response (Success)**:
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "Hello, Alice! Welcome to MCP."
      }
    ],
    "isError": false
  }
}
```

**Response (Error)**:
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "error": {
    "code": -32602,
    "message": "Invalid params: Missing 'name' parameter"
  }
}
```

---

#### 4. `resources/list`

**Purpose**: List available resources

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "method": "resources/list"
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "result": {
    "resources": [
      {
        "uri": "config://app",
        "name": "Application Configuration",
        "description": "Current application configuration",
        "mimeType": "application/json"
      }
    ]
  }
}
```

---

#### 5. `resources/read`

**Purpose**: Read a resource

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 5,
  "method": "resources/read",
  "params": {
    "uri": "config://app"
  }
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 5,
  "result": {
    "contents": [
      {
        "uri": "config://app",
        "mimeType": "application/json",
        "text": "{\"appName\": \"Rust MCP Server\", \"version\": \"1.0.0\", ...}"
      }
    ]
  }
}
```

---

#### 6. `prompts/list`

**Purpose**: List available prompts

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 6,
  "method": "prompts/list"
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 6,
  "result": {
    "prompts": [
      {
        "name": "review-code",
        "description": "Generates a prompt to ask the LLM to review code",
        "arguments": [
          {
            "name": "code",
            "description": "The code snippet to review",
            "required": true
          },
          {
            "name": "focus",
            "description": "Optional area of focus",
            "required": false
          }
        ]
      }
    ]
  }
}
```

---

#### 7. `prompts/get`

**Purpose**: Get a prompt

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 7,
  "method": "prompts/get",
  "params": {
    "name": "review-code",
    "arguments": {
      "code": "fn main() { println!(\"Hello!\"); }",
      "focus": "performance"
    }
  }
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 7,
  "result": {
    "description": "Requesting performance review for code snippet",
    "messages": [
      {
        "role": "user",
        "content": [
          {
            "type": "text",
            "text": "Please review the following code for potential issues and suggest improvements, focusing specifically on performance:\n\n```\nfn main() { println!(\"Hello!\"); }\n```"
          }
        ]
      }
    ]
  }
}
```

---

## Adding Custom Tools

### Step 1: Create Tool Module

```rust name=src/tools/custom_tool.rs
use super::{Tool, CallToolResult, TextContent, ToolHandler};
use serde_json::{json, Value};
use async_trait::async_trait;
use crate::utils::{Result, Error, Logger};

pub struct CustomTool {
    logger: Logger,
}

impl CustomTool {
    pub fn new() -> Self {
        Self {
            logger: Logger::new("CustomTool"),
        }
    }

    pub fn tool_definition() -> Tool {
        Tool {
            name: "custom-tool".to_string(),
            description: "Description of your custom tool".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "param1": {
                        "type": "string",
                        "description": "First parameter"
                    },
                    "param2": {
                        "type": "number",
                        "description": "Second parameter"
                    }
                },
                "required": ["param1"]
            }),
            annotations: Some(json!({
                "title": "Custom Tool",
                "readOnlyHint": true
            })),
        }
    }
}

impl Default for CustomTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ToolHandler for CustomTool {
    async fn call(&self, arguments: Value) -> Result<CallToolResult> {
        let param1 = arguments
            .get("param1")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::InvalidParams("Missing 'param1'".to_string()))?;

        let param2 = arguments
            .get("param2")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        self.logger.debug_with_context(
            "Tool called",
            &format!("param1={}, param2={}", param1, param2),
        );

        // Your custom logic here
        let result = format!("Processed: {} and {}", param1, param2);

        Ok(CallToolResult::success(vec![TextContent::new(result)]))
    }
}
```

### Step 2: Register in Module

Add to `src/tools/mod.rs`:

```rust
pub mod custom_tool;
pub use custom_tool::CustomTool;
```

### Step 3: Add to Server

In `src/server.rs`, update `handle_tools_list`:

```rust
async fn handle_tools_list(&self, _message: &Value) -> Result<Value> {
    self.logger.debug("Listing tools");

    let tools = vec![
        GreetingTool::tool_definition(),
        CalculatorTool::tool_definition(),
        WeatherTool::tool_definition(),
        CustomTool::tool_definition(),  // Add this
    ];

    Ok(json!({
        "tools": tools
    }))
}
```

And in `handle_tools_call`, add the match arm:

```rust
"custom-tool" => {
    let handler = CustomTool::new();
    handler.call(arguments).await?
}
```

### Step 4: Rebuild and Test

```bash
cargo build --release

# Test
echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"custom-tool","arguments":{"param1":"test","param2":42}}}' | ./target/release/mcp-server-rust
```

---

## Troubleshooting

### Issue: Server won't start

```bash
# Check if executable exists
ls -la target/release/mcp-server-rust

# Try building again
cargo clean
cargo build --release

# Check for errors
cargo check
```

### Issue: Connection refused

1. Verify the server is running
2. Check if another process is using the port
3. View logs:
   ```bash
   RUST_LOG=trace ./target/release/mcp-server-rust
   ```

### Issue: Tool not found

1. Verify tool is registered in `handle_tools_list`
2. Check tool name matches exactly
3. Verify match arm in `handle_tools_call`

### Issue: Invalid parameters error

1. Check argument names in JSON-RPC request
2. Verify types (string vs number)
3. Look at tool's `inputSchema`

### Issue: No output/frozen

1. Check if server is waiting for input
2. Verify stdin/stdout not blocked
3. Enable logging:
   ```bash
   RUST_LOG=trace ./target/release/mcp-server-rust
   ```

### Enable Debug Logging

```bash
# All modules
RUST_LOG=debug ./target/release/mcp-server-rust

# Specific module
RUST_LOG=rust_mcp_server::server=debug ./target/release/mcp-server-rust

# Very detailed
RUST_LOG=trace ./target/release/mcp-server-rust
```

### Test with MCP Inspector

```bash
# Terminal 1: Start server
./target/release/mcp-server-rust

# Terminal 2: Run inspector
npx @modelcontextprotocol/inspector stdio ./target/release/mcp-server-rust
```

---

## Security Considerations

### Input Validation

✅ All tool arguments are validated using JSON Schema  
✅ File paths are checked for traversal attempts  
✅ Tool parameters are type-checked  

### Output Sanitization

✅ Tool results are serialized safely as JSON  
✅ No raw command execution  
✅ Error messages don't expose internal details  

### Best Practices

1. **Never trust client input**
   ```rust
   // BAD
   let path = arguments.get("file").unwrap();
   
   // GOOD
   let path = arguments
       .get("file")
       .and_then(|v| v.as_str())
       .ok_or_else(|| Error::InvalidParams("..."))?;
   ```

2. **Validate file paths**
   ```rust
   fn validate_path(&self, filename: &str) -> Result<PathBuf> {
       let requested = self.base_dir.join(filename);
       let resolved = std::fs::canonicalize(&requested)?;
       if !resolved.starts_with(&self.base_dir) {
           return Err(Error::ResourceError("Path traversal".into()));
       }
       Ok(resolved)
   }
   ```

3. **Handle errors safely**
   ```rust
   match operation() {
       Ok(result) => Ok(CallToolResult::success(...)),
       Err(e) => {
           self.logger.error(&e.to_string());  // Log internally
           Ok(CallToolResult::error("Operation failed"))  // Generic response
       }
   }
   ```

4. **Set resource limits**
   ```rust
   const MAX_RESULT_SIZE: usize = 10_000_000;  // 10MB
   if result.len() > MAX_RESULT_SIZE {
       return Err(Error::InternalError("Result too large".into()));
   }
   ```

---

## Advanced Topics

### Creating a HTTP Transport

While the current implementation uses Stdio, you can add HTTP support:

```rust name=src/transport/http.rs
use crate::utils::Result;
use async_trait::async_trait;
use axum::{extract::Json, http::StatusCode, routing::post, Router};
use serde_json::Value;

#[async_trait]
pub async fn http_handler(
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
    // Handle JSON-RPC request
    (StatusCode::OK, Json(payload))
}

pub fn create_router() -> Router {
    Router::new()
        .route("/mcp", post(http_handler))
}
```

### Adding Persistent State

```rust
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ServerState {
    data: Arc<RwLock<HashMap<String, Value>>>,
}

impl ServerState {
    pub async fn get(&self, key: &str) -> Option<Value> {
        self.data.read().await.get(key).cloned()
    }
    
    pub async fn set(&self, key: String, value: Value) {
        self.data.write().await.insert(key, value);
    }
}
```

### Async Tool Execution

```rust
#[async_trait]
impl ToolHandler for AsyncTool {
    async fn call(&self, arguments: Value) -> Result<CallToolResult> {
        // Can use tokio::time::sleep, reqwest, etc.
        let result = tokio::time::timeout(
            std::time::Duration::from_secs(30),
            self.perform_async_work()
        ).await??;
        
        Ok(CallToolResult::success(vec![TextContent::new(result)]))
    }
}
```

---

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

---

## License

MIT License - see LICENSE file for details

---

## Resources

- **MCP Specification**: [modelcontextprotocol.io](https://modelcontextprotocol.io)
- **Rust Guide**: [doc.rust-lang.org](https://doc.rust-lang.org)
- **Tokio Docs**: [tokio.rs](https://tokio.rs)

---

## FAQ

**Q: Can I run multiple MCP servers?**  
A: Yes! Each application can manage multiple client-server connections.

**Q: Is this production-ready?**  
A: The core is stable, but review security considerations before production use.

**Q: Can I add database integration?**  
A: Absolutely! Use sqlx, tokio-postgres, etc. in your tool handlers.

**Q: How do I debug tools?**  
A: Enable logging with `RUST_LOG=debug` and use the MCP Inspector tool.

---

## Support

For issues and questions:
- Check this README and examples
- Enable debug logging
- Review MCP specification
- Open a GitHub issue

---

**Last Updated**: 2026-02-13  
**Version**: 1.0.0  
**Status**: Stable