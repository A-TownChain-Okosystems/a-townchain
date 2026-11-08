const fs = require('fs');
let content = fs.readFileSync('src/components/FranchiseFactoryView.tsx', 'utf8');

// The Title prop is already there. Wait actually, some tabs don't use the title prop or we can just let it be default.
// Let's modify the components that have a title prop to also have an assetTag that uses the same title string if possible.
// Finding: <TextGeneratorTab ... title="World Details & Ambient Lore KI"
// I will just use regex to add assetTag that equals the title prop.

content = content.replace(/(<(Text|Image|Video|Pipeline)GeneratorTab[^>]*?title=)("[^"]*")/g, '$1$3 assetTag=$3');

fs.writeFileSync('src/components/FranchiseFactoryView.tsx', content);
