// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
import * as fs from 'fs';

let content = fs.readFileSync('src/roadmapData.ts', 'utf8');

// adding an uncompleted goal to phase 2 just to see 50%
content = content.replace(
  /goals: \[\{ text: "Meilenstein M2: Stabiles Multi-Node-Testnet", completed: false \}\]/g,
  'goals: [{ text: "Entwicklung GossipSub Layer", completed: true }, { text: "Meilenstein M2: Stabiles Multi-Node-Testnet", completed: false }]'
);

// and for phase 3
content = content.replace(
  /goals: \[\{ text: "Meilenstein M3: KI-Kern produktiv", completed: false \}\]/g,
  'goals: [{ text: "Llama 3 Integration", completed: false }, { text: "Meilenstein M3: KI-Kern produktiv", completed: false }]'
);

fs.writeFileSync('src/roadmapData.ts', content);
