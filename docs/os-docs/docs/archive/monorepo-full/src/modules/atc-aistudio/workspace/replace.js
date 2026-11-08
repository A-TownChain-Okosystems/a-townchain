// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
const fs = require('fs');
const path = require('path');

function replaceInFile(filePath) {
  let content = fs.readFileSync(filePath, 'utf8');

  content = content.replace(/A-TownChain/g, 'System Network');
  content = content.replace(/ShivaOS/g, 'System Dashboard');
  content = content.replace(/ShivaCoreDev/g, 'CoreDev');
  content = content.replace(/KAI-OS/g, 'System OS');
  content = content.replace(/ATC_STANDARDS/g, 'CORE_STANDARDS');
  content = content.replace(/ATS_STANDARDS/g, 'APP_STANDARDS');
  content = content.replace(/ATC-/g, 'CORE-');
  content = content.replace(/ATS-/g, 'APP-');
  content = content.replace(/A-Town Standards/g, 'System Standards');
  content = content.replace(/A-Town/g, 'System');
  
  // Custom ATC and ATS replacements
  content = content.replace(/\(ATS\/ATC\)/g, '(APP/CORE)');
  content = content.replace(/\(ATC\)/g, '(CORE)');
  content = content.replace(/\(ATS\)/g, '(APP)');
  content = content.replace(/ ATC /g, ' CORE ');
  content = content.replace(/ ATS /g, ' APP ');
  
  // Specific occurrences
  content = content.replace(/Shivamon/g, 'Game Entities');
  content = content.replace(/Shiva/g, 'System');
  
  fs.writeFileSync(filePath, content, 'utf8');
}

const dir = path.join(__dirname, 'src');
const files = fs.readdirSync(dir);
files.forEach(file => {
  if (file.endsWith('.ts') || file.endsWith('.tsx')) {
    replaceInFile(path.join(dir, file));
  }
});
console.log('Done');
