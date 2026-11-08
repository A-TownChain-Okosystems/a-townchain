// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
import * as fs from 'fs';

let content = fs.readFileSync('src/wikiData.ts', 'utf8');

const categories = {
  'overview': 'Allgemein',
  'usps': 'Allgemein',
  'changelog': 'Allgemein',
  'architecture': 'Architektur & System',
  'structure': 'Architektur & System',
  'installation': 'Entwicklung',
  'dashboard': 'Entwicklung',
  'entwicklung': 'Entwicklung',
  'contracts': 'Blockchain & Token'
};

for (const [id, category] of Object.entries(categories)) {
  const regex = new RegExp(`(id:\\s*"${id}",\\n\\s*)`, 'g');
  content = content.replace(regex, `$1category: "${category}",\n    `);
}

fs.writeFileSync('src/wikiData.ts', content);
