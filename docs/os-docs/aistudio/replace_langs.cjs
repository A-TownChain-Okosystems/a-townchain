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
    content = content.replace(/TypeScript/g, 'atc-lang');
    content = content.replace(/Python(\s3\.10\+)?/g, 'atc-lang');
    content = content.replace(/Rust/g, 'atc-lang');
    content = content.replace(/Tailwind CSS/g, 'atc-css');
    content = content.replace(/JavaScript/g, 'atc-lang');
    content = content.replace(/WASM \/ atc-lang/g, 'WASM / atc-lang');
    content = content.replace(/atc-lang \/ Go/g, 'atc-lang');
    content = content.replace(/TS\/JS/g, 'atc-lang');
    
    fs.writeFileSync(file, content, 'utf8');
    console.log('Updated ' + file);
  }
});
