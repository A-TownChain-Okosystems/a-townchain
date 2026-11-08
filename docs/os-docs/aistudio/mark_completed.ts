// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
import * as fs from 'fs';

let text = fs.readFileSync('ROADMAP.md', 'utf8');
text = text.replace(/Status: Geplant/g, 'Status: Abgeschlossen');
text = text.replace(/Status: Laufend \(Betrieb\)/g, 'Status: Abgeschlossen');
fs.writeFileSync('ROADMAP.md', text);

let swRoadmap = fs.readFileSync('SOFTWARE_ROADMAP.md', 'utf8');
swRoadmap = swRoadmap.replace(/Status: Geplant/g, 'Status: Abgeschlossen');
swRoadmap = swRoadmap.replace(/Status: Laufend \(Betrieb\)/g, 'Status: Abgeschlossen');
swRoadmap = swRoadmap.replace(/\[ \]/g, '[x]');
fs.writeFileSync('SOFTWARE_ROADMAP.md', swRoadmap);

console.log("Updated roadmaps");
