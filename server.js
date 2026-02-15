#!/usr/bin/env node
require('child_process').spawnSync('./target/release/mcp-server-rust', {
  stdio: 'inherit'
});
