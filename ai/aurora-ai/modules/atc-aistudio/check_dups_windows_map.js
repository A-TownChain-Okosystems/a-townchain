// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
import fs from 'fs';
const c1 = fs.readFileSync('src/DesktopApp.tsx', 'utf-8');

const tmatches = c1.match(/^[ \t]+([a-zA-Z_][a-zA-Z0-9_]*):\s*\{(?=\s*\n\s*label:)/gm) || [];
const m1 = tmatches.map(m => m.split(':')[0].trim());

const seen = new Set();
const dups = new Set();
for (const id of m1) {
  if (seen.has(id)) dups.add(id);
  seen.add(id);
}
console.log('Duplicates in WINDOWS_MAP keys:', Array.from(dups));
