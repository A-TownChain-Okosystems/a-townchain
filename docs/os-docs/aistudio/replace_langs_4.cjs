const fs = require('fs');
const files = [
  'src/components/AtcLangArchitectureView.tsx',
  'src/components/GitOpsView.tsx',
  'src/components/ProjectHubView.tsx',
  'src/components/SoftwareAuditView.tsx',
  'src/components/StructureView.tsx',
  'src/components/TechDocsView.tsx',
  'src/components/TodoView.tsx'
];

files.forEach(file => {
  if (fs.existsSync(file)) {
    let content = fs.readFileSync(file, 'utf8');
    
    // Replace names
    content = content.replace(/RUST/g, 'atc-lang');
    content = content.replace(/Rust/g, 'atc-lang');
    content = content.replace(/TypeScript/g, 'atc-lang');
    content = content.replace(/JavaScript/g, 'atc-lang');
    content = content.replace(/javascript/g, 'atc-lang');
    
    fs.writeFileSync(file, content, 'utf8');
    console.log('Cleaned ' + file);
  }
});
