// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
import fs from 'fs';
const c1 = fs.readFileSync('src/DesktopApp.tsx', 'utf-8');
const c2 = fs.readFileSync('src/components/StructureView.tsx', 'utf-8');
const c3 = fs.readFileSync('src/components/TechTreeView.tsx', 'utf-8');

const tmatches = c1.match(/id:\s*['"]([^'"]+)['"]/g) || [];
const s1matches = c2.match(/id:\s*['"]([^'"]+)['"]/g) || [];
const s2matches = c3.match(/id:\s*['"]([^'"]+)['"]/g) || [];

const m1 = tmatches.map(m => m.split(/['"]/)[1]);
const m2 = s1matches.map(m => m.split(/['"]/)[1]);
const m3 = s2matches.map(m => m.split(/['"]/)[1]);

const all = [...m1, ...m2, ...m3];

const seen = new Set();
const dups = new Set();
for (const id of all) {
  if (seen.has(id)) dups.add(id);
  seen.add(id);
}
console.log('Duplicates across the three:', Array.from(dups));
