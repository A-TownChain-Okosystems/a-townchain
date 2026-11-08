// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
import fs from 'fs';
import path from 'path';

function walk(dir, callback) {
  fs.readdirSync(dir).forEach(f => {
    let dirPath = path.join(dir, f);
    let isDirectory = fs.statSync(dirPath).isDirectory();
    isDirectory ? walk(dirPath, callback) : callback(dirPath);
  });
}

walk('./src', function(filePath) {
  if (filePath.endsWith('.tsx') || filePath.endsWith('.ts')) {
    let content = fs.readFileSync(filePath, 'utf8');
    let newContent = content
      .replace(/KAI-OS/gi, 'ATC-OS')
      .replace(/KAI/gi, 'ATC')
      .replace(/kai_kernel/g, 'atc_kernel')
      .replace(/Shivamon/gi, 'ATC Asset')
      .replace(/shiva/gi, 'atc')
      .replace(/user@kai-os/g, 'user@atc-os')
      .replace(/0xKAI/g, '0xATC')
      .replace(/ATCAssetView/g, 'ATCAssetView'); // Just to be safe
    
    // Also rename ShivamonView to ATCAssetView in text
    newContent = newContent.replace(/ShivamonView/gi, 'ATCAssetView');
    newContent = newContent.replace(/ShivamonView\.tsx/gi, 'ATCAssetView.tsx');
    newContent = newContent.replace(/shivamon/gi, 'atcasset');
    
    if (content !== newContent) {
      fs.writeFileSync(filePath, newContent);
      console.log('Updated ' + filePath);
    }
  }
});
