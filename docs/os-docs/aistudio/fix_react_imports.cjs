const fs = require('fs');
const files = [
  'src/components/RescueSystemView.tsx',
  'src/components/TechDocsView.tsx'
];

files.forEach(file => {
  if (fs.existsSync(file)) {
    let content = fs.readFileSync(file, 'utf8');
    
    content = content.replace(/import atc-ui, \{/g, 'import React, {');
    content = content.replace(/import atc-ui from/g, 'import React from');
    content = content.replace(/import atc-ui /g, 'import React ');
    
    fs.writeFileSync(file, content, 'utf8');
    console.log('Fixed imports in ' + file);
  }
});
