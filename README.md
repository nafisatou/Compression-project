# Compression-project
A compression tool implementing Run-Length Encoding (RLE) and Simplified LZ77 algorithms in both Rust and JavaScript.
Project Structure

compression-project/
├── rust-compressor/
│   ├── src/
│   │   ├── main.rs
│   │   ├── rle.rs
│   │   └── lz.rs
│   ├── Cargo.toml
│   └── Dockerfile
├── js-compressor/
│   ├── index.js
│   ├── rle.js
│   ├── lz.js
│   └── package.json
    ├── test/
│   └── Dockerfile
└── README.md

To try out this project to test it functionalities, do;

git clone <repo-url>

Then,

cd <repo-name>

To pull the rust and js-compressor images, run the two commands below;

docker pull ghcr.io/nafisatou/rust-compressor:latest

docker pull ghcr.io/nafisatou/js-compressor:latest

To create aliases for easier use in the compression process, do;

    alias rust-compress="docker run -v $(pwd):/data rust-compressor"
    alias js-compress="docker run -v $(pwd):/data js-compressor"

After cloning the project, to test the rust-compressor functionality, do;

cd rust-compressor

Then,

cargo build --release && cargo test

To test the js-compressor functionality, do;

cd js-compressor

Then,

npm install &&  npm test

To test automatic algorithm selection, do;

    rust-compress compress input.txt output.compressed
    js-compress compress input.txt output.compressed

To test specific algorithm selection, do;

    rust-compress compress --algorithm rle input.txt output.rle
    js-compress compress -a lz input.txt output.lz

To decompress the files, do;

    rust-compress decompress input.compressed output.txt
    js-compress decompress input.compressed output.txt
