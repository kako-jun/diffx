#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const https = require('https');
const { execSync } = require('child_process');

const DIFFX_VERSION = '0.3.0';
const BINARY_DIR = path.join(__dirname, '..', 'bin');

function getPlatform() {
  const platform = process.platform;
  const arch = process.arch;
  
  if (platform === 'win32') {
    return 'diffx-windows-x86_64.zip';
  } else if (platform === 'darwin') {
    if (arch === 'arm64') {
      return 'diffx-macos-aarch64.tar.gz';
    } else {
      return 'diffx-macos-x86_64.tar.gz';
    }
  } else if (platform === 'linux') {
    return 'diffx-linux-x86_64.tar.gz';
  } else {
    throw new Error(`Unsupported platform: ${platform}-${arch}`);
  }
}

function downloadFile(url, dest) {
  return new Promise((resolve, reject) => {
    console.log(`Downloading diffx binary from ${url}...`);
    const file = fs.createWriteStream(dest);
    
    https.get(url, (response) => {
      if (response.statusCode === 302 || response.statusCode === 301) {
        // Follow redirect
        downloadFile(response.headers.location, dest).then(resolve).catch(reject);
        return;
      }
      
      if (response.statusCode !== 200) {
        reject(new Error(`HTTP ${response.statusCode}: ${response.statusMessage}`));
        return;
      }
      
      response.pipe(file);
      
      file.on('finish', () => {
        file.close();
        resolve();
      });
      
      file.on('error', (err) => {
        fs.unlink(dest, () => {}); // Delete the file async
        reject(err);
      });
    }).on('error', (err) => {
      reject(err);
    });
  });
}

async function extractArchive(archivePath, extractDir) {
  if (archivePath.endsWith('.zip')) {
    // Use unzip for Windows
    if (process.platform === 'win32') {
      execSync(`powershell -command "Expand-Archive -Path '${archivePath}' -DestinationPath '${extractDir}' -Force"`, { stdio: 'inherit' });
    } else {
      execSync(`unzip -o "${archivePath}" -d "${extractDir}"`, { stdio: 'inherit' });
    }
  } else if (archivePath.endsWith('.tar.gz')) {
    execSync(`tar -xzf "${archivePath}" -C "${extractDir}"`, { stdio: 'inherit' });
  }
}

async function main() {
  try {
    // Skip download if binary already exists
    const binaryName = process.platform === 'win32' ? 'diffx.exe' : 'diffx';
    const binaryPath = path.join(BINARY_DIR, binaryName);
    
    if (fs.existsSync(binaryPath)) {
      console.log('diffx binary already exists, skipping download.');
      return;
    }
    
    const platformFile = getPlatform();
    const downloadUrl = `https://github.com/kako-jun/diffx/releases/download/v${DIFFX_VERSION}/${platformFile}`;
    
    // Create bin directory
    if (!fs.existsSync(BINARY_DIR)) {
      fs.mkdirSync(BINARY_DIR, { recursive: true });
    }
    
    // Download archive
    const archivePath = path.join(BINARY_DIR, platformFile);
    await downloadFile(downloadUrl, archivePath);
    
    console.log('Extracting binary...');
    await extractArchive(archivePath, BINARY_DIR);
    
    // Clean up archive
    fs.unlinkSync(archivePath);
    
    // Make binary executable on Unix systems
    if (process.platform !== 'win32') {
      fs.chmodSync(binaryPath, '755');
    }
    
    console.log(`SUCCESS: diffx binary installed successfully at ${binaryPath}`);
    
  } catch (error) {
    console.error('ERROR: Failed to download diffx binary:', error.message);
    console.error('You may need to install diffx manually from: https://github.com/kako-jun/diffx/releases');
    // Don't fail the installation, just warn
    process.exit(0);
  }
}

if (require.main === module) {
  main();
}