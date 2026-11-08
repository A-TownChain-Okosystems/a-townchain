// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
const fs = require('fs');
let s = fs.readFileSync('src/wikiData.ts', 'utf8');
s = s.replace(/    category: .*\n    network:/g, '    network:');
fs.writeFileSync('src/wikiData.ts', s);
