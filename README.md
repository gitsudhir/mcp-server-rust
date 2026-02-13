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
9. [Integrating with Local LLMs](#integrating-with-local-llms)
   - [Claude Desktop](#claude-desktop)
   - [Ollama](#ollama)
   - [LM Studio](#lm-studio)
   - [Custom LLM Application](#custom-llm-application)
10. [API Reference](#api-reference)
11. [Adding Custom Tools](#adding-custom-tools)
12. [Troubleshooting](#troubleshooting)
13. [Security Considerations](#security-considerations)

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

## Integrating with Local LLMs

### Option 1: Claude Desktop

Claude Desktop is a native application that supports MCP servers.

#### Installation

1. Download Claude Desktop from [Anthropic's website](https://claude.ai/download)
2. Install and launch the application

#### Configuration

1. **Find Config File**:
   - **macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
   - **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`
   - **Linux**: `~/.config/Claude/claude_desktop_config.json`

2. **Edit Configuration**:
   ```json
   {
     "mcpServers": {
       "mcp-server-rust": {
         "command": "/full/path/to/target/release/mcp-server-rust",
         "args": [],
         "env": {
           "RUST_LOG": "debug"
         }
       }
     }
   }
   ```

3. **Restart Claude Desktop**
   - Close and reopen the application

4. **Verify Integration**:
   - Look for the server indicator in Claude Desktop
   - Check logs: `tail -f ~/Library/Logs/Claude/mcp*.log`

#### Using Tools in Claude

Once integrated, you can ask Claude:

- "Say hello to Alice" → Calls `greet` tool
- "Calculate my BMI: 70kg, 1.75m tall" → Calls `calculate-bmi` tool
- "What's the weather in London?" → Calls `fetch-weather` tool
- "Review this code for performance issues: [code]" → Generates `review-code` prompt

---

### Option 2: Ollama

Ollama runs LLMs locally on your machine.

#### Installation

1. **Download Ollama**: [ollama.ai](https://ollama.ai)
2. **Install** for your OS
3. **Verify**:
   ```bash
   ollama --version
   ```

#### Setup

1. **Start Ollama Server**:
   ```bash
   ollama serve
   ```
   Default: `http://localhost:11434`

2. **Pull a Model** (in another terminal):
   ```bash
   # Pull Mistral model
   ollama pull mistral
   
   # Or Llama2
   ollama pull llama2
   
   # Or Neural Chat
   ollama pull neural-chat
   ```

3. **Create a Python Client** to interact with MCP Server:

```python name=ollama_mcp_client.py
#!/usr/bin/env python3
"""
Client to interact with Rust MCP Server and Ollama LLM
"""

import json
import subprocess
import sys
import requests
from typing import Any, Optional

class MCPClient:
    def __init__(self, server_path: str):
        """Initialize MCP client with server process"""
        self.server_path = server_path
        self.process = None
        self.message_id = 0
    
    def start(self):
        """Start the MCP server"""
        self.process = subprocess.Popen(
            [self.server_path],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            bufsize=1
        )
        print("✓ MCP Server started")
    
    def send_request(self, method: str, params: dict = None) -> dict:
        """Send JSON-RPC request to server"""
        self.message_id += 1
        request = {
            "jsonrpc": "2.0",
            "id": self.message_id,
            "method": method,
            "params": params or {}
        }
        
        # Send request
        request_str = json.dumps(request)
        self.process.stdin.write(request_str + "\n")
        self.process.stdin.flush()
        
        # Read response
        response_str = self.process.stdout.readline()
        if not response_str:
            raise Exception("Server disconnected")
        
        return json.loads(response_str)
    
    def call_tool(self, tool_name: str, arguments: dict) -> str:
        """Call a tool and return result"""
        response = self.send_request("tools/call", {
            "name": tool_name,
            "arguments": arguments
        })
        
        if "error" in response:
            return f"Error: {response['error']['message']}"
        
        result = response.get("result", {})
        content = result.get("content", [])
        if content and "text" in content[0]:
            return content[0]["text"]
        return str(result)
    
    def list_tools(self) -> list:
        """List available tools"""
        response = self.send_request("tools/list")
        return response.get("result", {}).get("tools", [])
    
    def get_prompt(self, prompt_name: str, arguments: dict = None) -> str:
        """Get a prompt"""
        response = self.send_request("prompts/get", {
            "name": prompt_name,
            "arguments": arguments or {}
        })
        
        if "error" in response:
            return f"Error: {response['error']['message']}"
        
        result = response.get("result", {})
        messages = result.get("messages", [])
        if messages and "content" in messages[0]:
            return messages[0]["content"][0].get("text", "")
        return str(result)
    
    def stop(self):
        """Stop the server"""
        if self.process:
            self.process.terminate()
            self.process.wait()
            print("✓ MCP Server stopped")

class OllamaClient:
    def __init__(self, model: str = "mistral", base_url: str = "http://localhost:11434"):
        """Initialize Ollama client"""
        self.model = model
        self.base_url = base_url
        self.conversation_history = []
    
    def chat(self, message: str, context: Optional[str] = None) -> str:
        """Send message to Ollama and get response"""
        # Add context if provided
        if context:
            message = f"{context}\n\nUser: {message}"
        
        self.conversation_history.append({
            "role": "user",
            "content": message
        })
        
        try:
            response = requests.post(
                f"{self.base_url}/api/chat",
                json={
                    "model": self.model,
                    "messages": self.conversation_history,
                    "stream": False
                }
            )
            
            if response.status_code != 200:
                return f"Error: {response.status_code} - {response.text}"
            
            result = response.json()
            assistant_message = result.get("message", {}).get("content", "")
            
            self.conversation_history.append({
                "role": "assistant",
                "content": assistant_message
            })
            
            return assistant_message
        
        except requests.exceptions.ConnectionError:
            return "Error: Cannot connect to Ollama. Make sure Ollama is running on http://localhost:11434"
        except Exception as e:
            return f"Error: {str(e)}"

def main():
    """Main interaction loop"""
    import os
    
    # Get server path
    server_path = os.environ.get("MCP_SERVER_PATH", "./target/release/mcp-server-rust")
    
    if not os.path.exists(server_path):
        print(f"Error: MCP server not found at {server_path}")
        print("Build with: cargo build --release")
        return
    
    # Initialize clients
    mcp = MCPClient(server_path)
    ollama = OllamaClient()
    
    mcp.start()
    
    print("\n" + "="*60)
    print("Rust MCP Server + Ollama Local LLM Integration")
    print("="*60)
    print("\nAvailable Tools:")
    
    tools = mcp.list_tools()
    for tool in tools:
        print(f"  • {tool['name']}: {tool['description']}")
    
    print("\nCommands:")
    print("  /tools       - List available tools")
    print("  /greet NAME  - Greet someone")
    print("  /bmi W H     - Calculate BMI (weight in kg, height in m)")
    print("  /weather CITY - Get weather")
    print("  /review CODE - Review code")
    print("  /model MODEL - Switch Ollama model")
    print("  /quit        - Exit")
    print("\nOr just chat naturally and the AI will decide which tools to use!\n")
    
    try:
        while True:
            user_input = input("You: ").strip()
            
            if not user_input:
                continue
            
            if user_input.startswith("/"):
                # Handle commands
                parts = user_input.split(maxsplit=1)
                command = parts[0][1:]
                args = parts[1] if len(parts) > 1 else ""
                
                if command == "quit":
                    print("Goodbye!")
                    break
                
                elif command == "tools":
                    tools = mcp.list_tools()
                    print("\nAvailable Tools:")
                    for tool in tools:
                        print(f"  • {tool['name']}: {tool['description']}")
                    print()
                
                elif command == "greet":
                    if args:
                        result = mcp.call_tool("greet", {"name": args})
                        print(f"Server: {result}\n")
                    else:
                        print("Usage: /greet NAME\n")
                
                elif command == "bmi":
                    parts = args.split()
                    if len(parts) >= 2:
                        try:
                            weight = float(parts[0])
                            height = float(parts[1])
                            result = mcp.call_tool("calculate-bmi", {
                                "weightKg": weight,
                                "heightM": height
                            })
                            print(f"Server: {result}\n")
                        except ValueError:
                            print("Usage: /bmi WEIGHT HEIGHT (numbers)\n")
                    else:
                        print("Usage: /bmi WEIGHT HEIGHT\n")
                
                elif command == "weather":
                    if args:
                        result = mcp.call_tool("fetch-weather", {"city": args})
                        print(f"Server: {result}\n")
                    else:
                        print("Usage: /weather CITY\n")
                
                elif command == "review":
                    if args:
                        result = mcp.get_prompt("review-code", {"code": args})
                        print(f"Prompt: {result}\n")
                    else:
                        print("Usage: /review CODE\n")
                
                elif command == "model":
                    if args:
                        ollama.model = args
                        ollama.conversation_history = []
                        print(f"Model switched to: {args}\n")
                    else:
                        print(f"Current model: {ollama.model}\n")
                
                else:
                    print(f"Unknown command: /{command}\n")
            
            else:
                # Natural conversation
                print(f"\nAssistant (thinking about available tools: greet, calculate-bmi, fetch-weather, review-code)...\n")
                response = ollama.chat(user_input, context="You have access to MCP tools. Consider using them when relevant.")
                print(f"Assistant: {response}\n")
    
    finally:
        mcp.stop()

if __name__ == "__main__":
    main()
```

#### Usage

```bash
# Make script executable
chmod +x ollama_mcp_client.py

# Run with MCP server
MCP_SERVER_PATH="./target/release/mcp-server-rust" python3 ollama_mcp_client.py
```

---

### Option 3: LM Studio

LM Studio provides a user-friendly GUI for running local LLMs.

#### Installation

1. Download from [lmstudio.ai](https://lmstudio.ai)
2. Install for your OS
3. Launch the application

#### Setup

1. **Download a Model**:
   - Open LM Studio
   - Search for a model (e.g., Mistral, Llama 2)
   - Click "Download"

2. **Start Local Server**:
   - Click the "Local Server" tab
   - Select your downloaded model
   - Click "Start Server"
   - Default endpoint: `http://localhost:1234`

3. **Create Integration Script**:

```python name=lmstudio_mcp_integration.py
#!/usr/bin/env python3
"""
Integration script for LM Studio + Rust MCP Server
"""

import json
import subprocess
import requests
from typing import Optional

class MCPLMStudioIntegration:
    def __init__(self, mcp_server_path: str, lm_studio_url: str = "http://localhost:1234"):
        self.mcp_server_path = mcp_server_path
        self.lm_studio_url = lm_studio_url
        self.mcp_process = None
    
    def start_mcp_server(self):
        """Start MCP server"""
        self.mcp_process = subprocess.Popen(
            [self.mcp_server_path],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            bufsize=1
        )
        print("✓ MCP Server started")
    
    def call_tool(self, tool_name: str, arguments: dict) -> str:
        """Call MCP tool"""
        request = {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {
                "name": tool_name,
                "arguments": arguments
            }
        }
        
        request_str = json.dumps(request)
        self.mcp_process.stdin.write(request_str + "\n")
        self.mcp_process.stdin.flush()
        
        response_str = self.mcp_process.stdout.readline()
        response = json.loads(response_str)
        
        if "error" in response:
            return f"Error: {response['error']['message']}"
        
        content = response.get("result", {}).get("content", [])
        return content[0]["text"] if content else str(response)
    
    def chat_with_lm_studio(self, message: str, use_tools: bool = True) -> str:
        """Send message to LM Studio with tool awareness"""
        
        # If use_tools, get available tools
        tool_info = ""
        if use_tools:
            tool_info = """
Available tools:
- greet(name): Greets someone
- calculate-bmi(weightKg, heightM): Calculates BMI
- fetch-weather(city): Gets weather info
- review-code(code, focus): Reviews code

When the user asks for something these tools can do, use them!
"""
        
        try:
            response = requests.post(
                f"{self.lm_studio_url}/v1/chat/completions",
                json={
                    "model": "local-model",
                    "messages": [
                        {
                            "role": "system",
                            "content": f"You are a helpful assistant. {tool_info}"
                        },
                        {
                            "role": "user",
                            "content": message
                        }
                    ],
                    "temperature": 0.7,
                    "max_tokens": 512,
                    "stream": False
                }
            )
            
            if response.status_code != 200:
                return f"Error: {response.status_code}"
            
            result = response.json()
            return result["choices"][0]["message"]["content"]
        
        except requests.exceptions.ConnectionError:
            return "Error: Cannot connect to LM Studio. Make sure it's running on http://localhost:1234"
        except Exception as e:
            return f"Error: {str(e)}"
    
    def stop_mcp_server(self):
        """Stop MCP server"""
        if self.mcp_process:
            self.mcp_process.terminate()
            self.mcp_process.wait()
            print("✓ MCP Server stopped")

if __name__ == "__main__":
    integration = MCPLMStudioIntegration("./target/release/mcp-server-rust")
    integration.start_mcp_server()
    
    # Example interaction
    response = integration.chat_with_lm_studio("Say hello to Alice")
    print(f"Response: {response}")
    
    integration.stop_mcp_server()
```

---

### Option 4: Custom LLM Application

Create your own LLM integration:

```rust name=examples/custom_integration.rs
//! Example: Custom integration with Rust MCP Server
//! 
//! This example shows how to integrate the MCP server programmatically
//! in your own Rust application.

use rust_mcp_server::{McpServer, ServerConfig};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create MCP server
    let server = McpServer::new(ServerConfig::new("CustomApp", "1.0.0"));
    
    // Simulate client requests
    println!("=== Testing MCP Server Directly ===\n");
    
    // 1. Initialize
    let init_request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {"name": "CustomApp", "version": "1.0.0"}
        }
    });
    
    match server.handle_request(init_request).await {
        Ok(Some(response)) => println!("Initialize Response: {}\n", response),
        Ok(None) => println!("Initialize: No response (notification)\n"),
        Err(e) => println!("Error: {}\n", e),
    }
    
    // 2. List tools
    let list_tools_request = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list"
    });
    
    match server.handle_request(list_tools_request).await {
        Ok(Some(response)) => println!("Available Tools:\n{}\n", response),
        _ => {}
    }
    
    // 3. Call greet tool
    let greet_request = json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "greet",
            "arguments": {"name": "Alice"}
        }
    });
    
    match server.handle_request(greet_request).await {
        Ok(Some(response)) => println!("Greet Tool Response:\n{}\n", response),
        _ => {}
    }
    
    // 4. Call calculate-bmi tool
    let bmi_request = json!({
        "jsonrpc": "2.0",
        "id": 4,
        "method": "tools/call",
        "params": {
            "name": "calculate-bmi",
            "arguments": {"weightKg": 70.0, "heightM": 1.75}
        }
    });
    
    match server.handle_request(bmi_request).await {
        Ok(Some(response)) => println!("BMI Tool Response:\n{}\n", response),
        _ => {}
    }
    
    // 5. Get code review prompt
    let prompt_request = json!({
        "jsonrpc": "2.0",
        "id": 5,
        "method": "prompts/get",
        "params": {
            "name": "review-code",
            "arguments": {
                "code": "fn add(a: i32, b: i32) -> i32 { a + b }",
                "focus": "performance"
            }
        }
    });
    
    match server.handle_request(prompt_request).await {
        Ok(Some(response)) => println!("Code Review Prompt:\n{}\n", response),
        _ => {}
    }
    
    Ok(())
}
```

Run it:

```bash
cargo run --example custom_integration
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

### Issue: Connection refused (Claude Desktop)

1. Verify path in config file is correct
2. Check file permissions:
   ```bash
   chmod +x target/release/mcp-server-rust
   ```
3. View logs:
   ```bash
   tail -f ~/Library/Logs/Claude/mcp*.log
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
- **Ollama**: [ollama.ai](https://ollama.ai)
- **Claude Desktop**: [anthropic.com/claude/download](https://claude.ai/download)
- **LM Studio**: [lmstudio.ai](https://lmstudio.ai)

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

**Q: Can I use this with GPT-4?**  
A: Not directly (OpenAI doesn't support MCP), but you can create an adapter.

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

```

This comprehensive README includes:

✅ Complete installation instructions  
✅ Building and running the server  
✅ Integration with Claude Desktop, Ollama, and LM Studio  
✅ Full API reference with JSON-RPC examples  
✅ How to add custom tools  
✅ Troubleshooting guide  
✅ Security best practices  
✅ Advanced topics  
✅ FAQ section  

Save this as `README.md` in your project root!