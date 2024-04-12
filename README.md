# dict8r

dict8r is a command-line tool written in Rust for dictating text replacements in files using a dictionary file.

## Features
- Replace variables in files using a dictionary file.
- Support for dry-run mode to preview changes without modifying files.
- Recursive directory processing.
- File filtering by extension.

## Usage

### Installation

To build dict8r from source, ensure you have Rust installed on your system. Then, clone this repository and navigate to the project directory:

```bash
git clone https://github.com/bkovaluk/dict8r.git
cd dict8r
```

### Build

To build dict8r, run the following command:

```bash
cargo build --release
```

### Testing

To run tests, use the following command:

```bash
cargo test
```

### Running

To run dict8r against files, use the following command:

```bash
./target/release/dict8r <path> <dict-file> [options]
```

##### Options

- `--dry-run`: Perform a trial run with no changes made.
- `-r`, `--recursive`: Process directories recursively.
- `-f`, `--filter <EXTENSION>`: Filter files by extension. You can specify multiple extensions by repeating the flag.

##### Example

```
./target/release/dict8r /path/to/files dictionary.json --recursive -f txt,md --dry-run
```

This command will perform a dry run on all .txt and .md files in the specified directory and its subdirectories.
