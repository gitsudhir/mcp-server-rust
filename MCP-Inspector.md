# 📘 How to Run a Rust MCP Server in MCP Inspector

This guide explains how to run a Rust-based MCP (Model Context Protocol) server using the MCP Inspector for testing and debugging.

---

# 1️⃣ Prerequisites

Make sure you have:

* ✅ Rust installed (`cargo`)
* ✅ Node.js v20+ installed
* ✅ Your MCP Rust server project ready

Check versions:

```bash
node -v
cargo --version
```

---

# 2️⃣ Build Your Rust MCP Server

From your project root:

```bash
cargo build --release
```

Your compiled binary will be created at:

```
./target/release/<your-binary-name>
```

Example:

```
./target/release/mcp-server-rust
```

---

# 3️⃣ Create a Node Wrapper (Required for Inspector)

MCP Inspector expects a Node entrypoint.

Create a file named:

```
server.js
```

Add this content:

```js
#!/usr/bin/env node
require('child_process').spawnSync('./target/release/mcp-server-rust', {
  stdio: 'inherit'
});
```

Make it executable:

```bash
chmod +x server.js
```

---

# 4️⃣ Start MCP Inspector

Run:

```bash
DANGEROUSLY_OMIT_AUTH=true npx @modelcontextprotocol/inspector node server.js
```

What this does:

* Starts Inspector UI
* Starts proxy
* Launches your Rust server via STDIO

You will see:

```
MCP Inspector is up and running at http://127.0.0.1:6274
```

---

# 5️⃣ Open Inspector UI

Open your browser:

```
http://localhost:6274
```

---

# 6️⃣ Configure Connection in UI

Inside the Inspector:

### Transport Type

Select:

```
STDIO
```

### Command

```
node
```

### Arguments

```
server.js
```

Leave other fields empty unless needed.

Click:

```
Connect
```

You should see:

* Connected status
* Tools list populated
* Ability to run tools

---

# 7️⃣ Test Your MCP Server

Go to:

```
Tools → List Tools
```

Then:

```
Run Tool
```

You should see successful JSON responses.

---

# 8️⃣ Important Rules for Rust MCP Server

Your server must:

✅ Stay running
✅ Read JSON-RPC from stdin
✅ Write JSON-RPC to stdout
❌ Never print logs to stdout
✔ Print logs to stderr instead

If logs are printed to stdout, Inspector will break.

---

# 9️⃣ Optional: Run Without Disabling Auth

If you don’t use `DANGEROUSLY_OMIT_AUTH=true`, Inspector will generate a session token.

You must:

1. Copy the token from terminal
2. Paste it into:

   ```
   Configuration → Proxy Session Token
   ```
3. Click Connect

---

# 🔟 Common Errors & Fixes

### ❌ “Option '--env' argument is ambiguous”

Cause: Incorrect CLI usage
Fix: Use:

```bash
npx @modelcontextprotocol/inspector node server.js
```

---

### ❌ “Connection Error”

Cause: Missing proxy token
Fix: Add token or disable auth.

---

### ❌ Inspector connects but no tools

Cause: Rust server not implementing `tools/list` correctly.

---

# ✅ Final Working Command (Recommended)

```bash
cargo build --release
DANGEROUSLY_OMIT_AUTH=true npx @modelcontextprotocol/inspector node server.js
```

Then open:

```
http://localhost:6274
```

---

# 🎯 Result

You now have:

* A fully working Rust MCP server
* Connected via STDIO
* Testable using MCP Inspector
* Ready for Claude Desktop / Cursor integration


