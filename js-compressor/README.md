# Compression Project

A CLI tool written in **Rust** and **JavaScript** for compressing and decompressing files using `RLE` and `LZ` algorithms.

## 🚀 Features


- Compression and decompression via CLI
- Supports RLE and LZ
- Dockerized builds
- Benchmarked performance
- Written in Rust and Node.js

---

## 🦀 Rust CLI

### Usage:

```bash
cargo run --release -- compress --rle input.txt output.txt
cargo run --release -- decompress --lz output.txt decompressed.txt


## JS Compression Benchmark Report

### Methodology:
We tested two compression methods (`minify` and `rle`) on various JavaScript files with different sizes.

### Results:

| File              | Compression Method | Time Taken (s) | Original Size (bytes) | Compressed Size (bytes) | Compression Ratio (%) |
|-------------------|--------------------|----------------|-----------------------|-------------------------|-----------------------|
| sample1.js        | minify             | 0.5            | 1200                  | 800                     | 66.67%                |
| sample1.js        | rle                | 0.8            | 1200                  | 700                     | 58.33%                |
| largefile.js      | minify             | 2.1            | 15000                 | 9000                    | 60%                   |
| largefile.js      | rle                | 3.2            | 15000                 | 7500                    | 50%                   |

### Conclusion:
- **Minification** is faster than **RLE** but provides slightly less compression.
- **RLE** works better on larger files but takes longer to process.
- For smaller files, minification is the better choice, but for large files where size reduction is a priority, RLE is worth considering.

