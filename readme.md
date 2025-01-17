# ddupes
Struggling with duplicate files and directories? ddupes simplifies the process by computing checksums for both files and directories to swiftly identify and remove redundancies. Reclaim your storage space effortlessly with ddupes.

## Features
- Computes checksums for directories by aggregating the checksums of their contents
- TODO: Utilizes hash functions to identify unique multimedia files, including audio, video, and images
- TODO: Supports replacing deleted duplicates with symbolic links
- Efficiently removes duplicate directories while ensuring unique content is preserved
- Employs concurrency for faster and more efficient processing of directories and files

## Prerequisites
- Rust (for building the program)
- Cargo (Rust package manager)

## Installation
1. Clone the repository:
```sh
git clone https://github.com/jskinnerd/ddupes.git
cd ddupes
```

2. Build the project:
```sh
cargo build --release
```

## Usage
Provide the path of the directory you want to scan:
```sh
ddupes <path_to_scan>
```

## License
For more information and to view the license, visit:
http://www.jskinnerd.com/puniko/index.htm