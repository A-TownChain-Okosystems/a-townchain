// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
import * as fs from 'fs';

let content = fs.readFileSync('src/roadmapData.ts', 'utf8');

// Replace all goals: ["..."] with goals: [{ text: "...", completed: false }]
content = content.replace(/goals:\s*\["(.+?)"\]/g, 'goals: [{ text: "$1", completed: false }]');

// For phases with status "Abgeschlossen", set completed to true
content = content.replace(/status:\s*"Abgeschlossen"[\s\S]*?goals:\s*\[{\s*text:\s*"(.+?)",\s*completed:\s*false\s*}\]/g, (match, p1) => {
  return match.replace(/completed:\s*false/, 'completed: true');
});

fs.writeFileSync('src/roadmapData.ts', content);
