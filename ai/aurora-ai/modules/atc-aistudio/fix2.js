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
      .replace(/ATC-AssetView/g, 'ATCAssetView')
      .replace(/ATC-Asset/g, 'ATCAsset')
      .replace(/ATC Asset/g, 'ATCAsset')
      .replace(/ATC AssetView/g, 'ATCAssetView');
      
    if (content !== newContent) {
      fs.writeFileSync(filePath, newContent);
      console.log('Fixed syntax in ' + filePath);
    }
  }
});
