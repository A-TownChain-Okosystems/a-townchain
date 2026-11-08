const fs = require('fs');
const files = [
  'src/wikiData.ts'
];

files.forEach(file => {
  if (fs.existsSync(file)) {
    let content = fs.readFileSync(file, 'utf8');
    
    // Replace names
    content = content.replace(/```javascript/g, '```atc-lang');
    content = content.replace(/javascript/ig, 'atc-lang');
    
    fs.writeFileSync(file, content, 'utf8');
    console.log('Cleaned ' + file);
  }
});
