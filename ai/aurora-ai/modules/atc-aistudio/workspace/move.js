// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
import fs from 'fs';
import path from 'path';

const srcDir = '/workspace/src_tmp';
const destDir = '/workspace/src';

const files = fs.readdirSync(srcDir);
for (const file of files) {
  const srcFile = path.join(srcDir, file);
  const destFile = path.join(destDir, file);
  fs.renameSync(srcFile, destFile);
}
