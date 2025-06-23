# file_dig

A crate to help find files in a directory according to various criteria.

## Features

- Search the list of files in a directory according to given criteria.
- List of criteria :
    - File name
    - File size
    - File extension
- Search can be recursive or not

## Installation

```bash
cargo install file_dig
```

## Usage

```rust
use project_name;

fn main() {
    let criteria = file_dig::FindCriteria::new()
        .file_name(OsString::from("image"))
        .file_extension(OsString::from("png"))
        .file_size_minimum(674567)
        .file_size_maximum(12346782);
    let result = file_dig::find("tests_files", &criteria);
}
```

## Requirements

- Rust 1.x
- Cargo

## Building from Source

Clone the repository and build using Cargo:

```bash
git clone https://github.com/craftlion/file_dig.git
cd file_dig
cargo build --release
```

## Running Tests

```bash
cargo test
```

## License

This project is licensed under MIT - see the `LICENSE` file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.