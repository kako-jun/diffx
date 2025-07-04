#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

// Determine the platform-specific binary name
let binaryName = 'diffx';
if (process.platform === 'win32') {
  binaryName = 'diffx.exe';
}

// Construct the path to the binary
// In a real scenario, this would involve downloading the binary
// For now, we assume it's in a 'bin' directory relative to this script
const binaryPath = path.join(__dirname, 'bin', binaryName);

// Check if the binary exists
if (!fs.existsSync(binaryPath)) {
  console.error(`Error: Binary not found at ${binaryPath}`);
  console.error('Please ensure diffx is properly installed or built for your platform.');
  process.exit(1);
}

// Spawn the diffx process with arguments
const child = spawn(binaryPath, process.argv.slice(2), {
  stdio: 'inherit',
});

child.on('close', (code) => {
  process.exit(code);
});

child.on('error', (err) => {
  console.error(`Failed to start diffx: ${err.message}`);
  process.exit(1);
});
