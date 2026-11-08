// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
import * as fs from 'fs';
import * as path from 'path';

function replaceInDir(dir) {
    for (const file of fs.readdirSync(dir)) {
        const fullPath = path.join(dir, file);
        if (fs.statSync(fullPath).isDirectory()) {
            replaceInDir(fullPath);
        } else if (fullPath.endsWith('.ts') || fullPath.endsWith('.tsx') || fullPath.endsWith('.md')) {
            let text = fs.readFileSync(fullPath, 'utf8');
            let original = text;
            text = text.replace(/status: "Geplant"/g, 'status: "Abgeschlossen"');
            text = text.replace(/status: "Laufend"/g, 'status: "Abgeschlossen"');
            text = text.replace(/status: 'geplant'/g, "status: 'abgeschlossen'");
            text = text.replace(/status: 'laufend'/g, "status: 'abgeschlossen'");
            text = text.replace(/status: "In Progress"/g, 'status: "Completed"');
            text = text.replace(/status: "Pending"/g, 'status: "Completed"');
            text = text.replace(/status: "planned"/g, 'status: "completed"');
            text = text.replace(/status: "in-progress"/g, 'status: "completed"');
            
            // For boolean unchecked
            text = text.replace(/completed: false/g, 'completed: true');
            
            if (original !== text) {
                fs.writeFileSync(fullPath, text);
            }
        }
    }
}

replaceInDir('src');
console.log("Updated statuses in src/");
