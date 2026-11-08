// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
import fs from 'fs';
const content = fs.readFileSync('src/components/TechTreeView.tsx', 'utf-8');
const matches = content.match(/id:\s*['"]([^'"]+)['"]/g) || [];
const ids = matches.map(m => m.split(/['"]/)[1]);
const seen = new Set();
const dups = new Set();
for (const id of ids) {
  if (seen.has(id)) dups.add(id);
  seen.add(id);
}
console.log('Duplicates in TechTreeView:', Array.from(dups));
