const fs = require('fs');
const files = [
  'src/components/RescueSystemView.tsx',
  'src/components/TechDocsView.tsx',
  'SOFTWARE_ROADMAP.md',
  'ROADMAP.md'
];

files.forEach(file => {
  if (fs.existsSync(file)) {
    let content = fs.readFileSync(file, 'utf8');
    
    content = content.replace(/Node\.js/g, 'atc-runtime');
    content = content.replace(/npm/g, 'atpm');
    content = content.replace(/React/g, 'atc-ui');
    
    fs.writeFileSync(file, content, 'utf8');
    console.log('Cleaned ' + file);
  }
});
