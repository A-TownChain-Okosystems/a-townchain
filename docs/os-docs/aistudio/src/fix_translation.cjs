const fs = require('fs');
const filePath = 'src/components/NetworkExplorerView.tsx';
let data = fs.readFileSync(filePath, 'utf8');
data = data.replace('Type</span>', 'Typ</span>');
data = data.replace('Latency</span>', 'Latenz</span>');
data = data.replace('Uptime</span>', 'Betriebszeit</span>');
data = data.replace('Peer Links</span>', 'Peer-Verbindungen</span>');
data = data.replace('Sync</span>', 'Status</span>');
fs.writeFileSync(filePath, data, 'utf8');
