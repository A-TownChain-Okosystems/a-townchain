const fs = require('fs');
const files = [
  'SOFTWARE_ROADMAP.md',
  'ROADMAP.md',
  'src/ecosystemData.ts',
  'src/wikiData.ts',
  'src/App.tsx'
];

files.forEach(file => {
  if (fs.existsSync(file)) {
    let content = fs.readFileSync(file, 'utf8');
    
    // Replace names
    content = content.replace(/atc-lang\/Go/g, 'atc-lang');
    content = content.replace(/Go(?=[, \n])/g, 'atc-lang');
    content = content.replace(/C\+\+/g, 'atc-lang');
    content = content.replace(/Java(?=[, \n])/g, 'atc-lang');
    content = content.replace(/C#(?=[, \n])/g, 'atc-lang');
    
    fs.writeFileSync(file, content, 'utf8');
    console.log('Cleaned ' + file);
  }
});
