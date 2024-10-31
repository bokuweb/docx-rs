import fs from 'fs';
import cp from 'child_process';
import path from 'path';

const fileType = 'png';

const inputDir = process.env.OUTPUT_DIR ?? process.exit(1);
const files = fs.readdirSync(inputDir);

for (const file of files) {
  if (/\.docx$/.test(file)) {
    console.log(file);
    const spawn = cp.spawnSync('make', [
      'run',
      `INPUT=${path.join(inputDir, file)}`,
      `OUTPUT=${path.join(__dirname, 'png', file.replace(/\.docx$/, `.${fileType}`))}`,
    ]);
    console.log(spawn.stdout.toString());
    //break;
  }
}
