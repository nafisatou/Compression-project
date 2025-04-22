import fs from 'fs';
import { compress as compressRLE, decompress as decompressRLE } from './rle.js';
import { compress as compressLZ, decompress as decompressLZ } from './lz.js';

const [,, command, inputPath, outputPath, algoFlag] = process.argv;

if (!['compress', 'decompress'].includes(command)) {
    console.error('Invalid command. Use "compress" or "decompress".');
    process.exit(1);
}

if (!inputPath || !outputPath || !algoFlag) {
    console.error('Usage: node index.js <compress|decompress> <input> <output> --rle|--lz');
    process.exit(1);
}

const data = fs.readFileSync(inputPath);
let output;

const isRLE = algoFlag === '--rle';
const isLZ = algoFlag === '--lz';

if (isRLE) {
    output = command === 'compress' ? compressRLE(data) : decompressRLE(data);
} else if (isLZ) {
    output = command === 'compress' ? compressLZ(data) : decompressLZ(data);
} else {
    console.error('Unknown algorithm flag. Use --rle or --lz');
    process.exit(1);
}

fs.writeFileSync(outputPath, output);
console.log(`${command}ion done! Output: ${outputPath}`);
