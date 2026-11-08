// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
import fs from 'fs';
import path from 'path';

const srcDir = './workspace/src_tmp';
const destDir = './src';

const files = fs.readdirSync(srcDir);
for (const file of files) {
  fs.renameSync(path.join(srcDir, file), path.join(destDir, file));
}
