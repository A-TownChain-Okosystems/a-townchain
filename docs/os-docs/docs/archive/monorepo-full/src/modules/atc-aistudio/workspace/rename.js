// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
import fs from 'fs';
import path from 'path';

function replaceInFile(filePath) {
  const content = fs.readFileSync(filePath, 'utf8');
  let newContent = content
    .replace(/GlobusOs/g, 'A-TownOS')
    .replace(/GlobusOS/g, 'A-TownOS')
    .replace(/Globus Consensus/g, 'ATC Consensus')
    .replace(/Globus VM/g, 'ATC VM')
    .replace(/GateToHell/g, 'A-TownChain')
    .replace(/globus_vm/g, 'atc_vm')
    .replace(/gth-/g, 'atc-'); // CSS prefixes

  if (content !== newContent) {
    fs.writeFileSync(filePath, newContent, 'utf8');
    console.log(`Updated ${filePath}`);
  }
}

function traverseDir(dir) {
  const files = fs.readdirSync(dir);
  for (const file of files) {
    const fullPath = path.join(dir, file);
    if (fs.statSync(fullPath).isDirectory()) {
      if (!['node_modules', 'dist', '.git'].includes(file)) {
        traverseDir(fullPath);
      }
    } else {
      if (['.ts', '.tsx', '.html', '.css', '.json', '.md'].includes(path.extname(fullPath))) {
        replaceInFile(fullPath);
      }
    }
  }
}

traverseDir('./src');
replaceInFile('./index.html');
replaceInFile('./package.json');
try { replaceInFile('./metadata.json'); } catch(e){}

