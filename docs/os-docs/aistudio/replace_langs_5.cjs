const fs = require('fs');
const files = [
  'SOFTWARE_ROADMAP.md',
  'src/components/StructureView.tsx',
  'src/components/TechDocsView.tsx'
];

files.forEach(file => {
  if (fs.existsSync(file)) {
    let content = fs.readFileSync(file, 'utf8');
    
    // Replace names
    content = content.replace(/Solidity/g, 'atc-lang');
    content = content.replace(/solidity/g, 'atc-lang');
    content = content.replace(/\.sol/g, '.atc');
    
    fs.writeFileSync(file, content, 'utf8');
    console.log('Cleaned ' + file);
  }
});
