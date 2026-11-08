const fs = require('fs');
let content = fs.readFileSync('src/components/FranchiseFactoryView.tsx', 'utf8');

content = content.replace(/<TextGeneratorTab /g, '<TextGeneratorTab onAssetGenerated={handleAssetGenerated} ');
content = content.replace(/<ImageGeneratorTab /g, '<ImageGeneratorTab onAssetGenerated={handleAssetGenerated} ');
content = content.replace(/<VideoGeneratorTab /g, '<VideoGeneratorTab onAssetGenerated={handleAssetGenerated} ');
content = content.replace(/<PipelineGeneratorTab /g, '<PipelineGeneratorTab onAssetGenerated={handleAssetGenerated} ');
content = content.replace(/<Ai[A-Za-z0-9]+EngineTab /g, function(m) { return m + 'onAssetGenerated={handleAssetGenerated} '; });
content = content.replace(/<AiCharacterBioTab /g, '<AiCharacterBioTab onAssetGenerated={handleAssetGenerated} ');

fs.writeFileSync('src/components/FranchiseFactoryView.tsx', content);
