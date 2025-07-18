# Alchemy

> Convert stuff into other stuff

## Description

Alchemy is a powerful command-line tool for encoding, decoding, hashing, and various data transformations. It's designed to be fast, flexible, and easy to use for all your data manipulation needs.

## Features

- **Encoding/Decoding**: Convert between hex, base64, base58, binary, UTF-8, ASCII, and more
- **Classification**: Automatically detect input encoding formats
- **Hashing**: Support for SHA256, SHA512, MD5, Blake2, Keccak256, and more
- **Array Operations**: Chunk, flatten, reverse, rotate arrays
- **Data Generation**: Generate random data in various formats
- **Padding**: Pad data to specific sizes
- **Arbitrary Precision**: Handle large numbers with GMP/MPFR support

## Installation

### Download Binary (Recommended)

Download the latest release for your platform from the [releases page](https://github.com/rubenduburck/alchemy/releases).

```bash
# Download and install (example for Linux x64)
curl -L https://github.com/rubenduburck/alchemy/releases/latest/download/alchemy-x86_64-linux -o alchemy
chmod +x alchemy
sudo mv alchemy /usr/local/bin/
```

### Build from Source

Requires Rust toolchain and GMP/MPFR libraries:

```bash
# Install dependencies (Ubuntu/Debian)
sudo apt-get install libgmp-dev libmpfr-dev libmpc-dev

# Install dependencies (macOS)
brew install gmp mpfr

# Build and install
cargo build --release
sudo cp target/release/alchemy /usr/local/bin/
```

### Using Make

```bash
# Download latest binary for your platform
make install

# Build from source
make build

# Build for specific target
make build-target TARGET=aarch64-unknown-linux-gnu
```

## Usage

### Basic Commands

```bash
# Classify input encoding
alchemy classify "0x1234"

# Convert between encodings
alchemy convert --from hex --to base64 "0x1234"

# Auto-detect input encoding and convert
alchemy convert --to base64 "0x1234"

# Convert with short flags
alchemy convert -i hex -o base64 "0x1234"
```

### Hashing

```bash
# Hash with SHA256 (default)
alchemy hash sha256 "hello world"

# Other hash algorithms
alchemy hash md5 "data"
alchemy hash blake2 "0x1234"
alchemy hash keccak256 "test"
```

### Array Operations

```bash
# Chunk array into groups of 2
alchemy chunk-array -c 2 "[1,2,3,4,5,6]"
# Output: [[1,2],[3,4],[5,6]]

# Flatten nested arrays
alchemy flatten-array "[[1,2],[3,4]]"
# Output: [1,2,3,4]

# Reverse array at depth 1
alchemy reverse-array -d 1 "[1,2,3,4]"
# Output: [4,3,2,1]

# Rotate array by 2 positions
alchemy rotate-array -r 2 "[1,2,3,4]"
# Output: [3,4,1,2]
```

### Data Generation

```bash
# Generate 32 bytes of random hex
alchemy generate -e hex -b 32

# Generate random base64 data
alchemy random -e base64 -b 16

# Generate random integer
alchemy generate -e int -b 8
```

### Padding

```bash
# Pad hex data to 32 bytes on the left
alchemy pad-left -p 32 "0x1234"

# Pad on the right
alchemy pad-right -p 32 "0x1234"
```

### Advanced Usage

```bash
# Pipe data through alchemy
echo "hello world" | alchemy convert -o hex

# Chain operations
alchemy convert -o hex "test" | alchemy hash sha256

# Use with other tools
cat file.bin | alchemy convert -o base64 > file.b64
```

## Supported Encodings

- **hex**: Hexadecimal (with or without 0x prefix)
- **bytes**: Byte arrays like [0x12, 0x34]
- **int**: Decimal integers
- **bin**: Binary (0b prefix optional)
- **base{2-36}**: Base N encoding
- **base58**: Base58 encoding
- **base64**: Base64 encoding
- **utf8**: UTF-8 text
- **utf16**: UTF-16 text
- **ascii**: ASCII text

## Supported Hash Algorithms

- **MD5**: `md5`
- **SHA1**: `sha1`
- **SHA2 Family**: `sha256`, `sha384`, `sha512`
- **SHA3 Family**: `sha3-256`, `sha3-384`, `sha3-512`
- **Keccak**: `keccak256`, `keccak512`
- **Blake2**: `blake2b`, `blake2s`

## Integration

### Vim/Neovim Plugin

For seamless integration with Vim/Neovim, use the [vim-alchemy](https://github.com/rubenduburck/vim-alchemy) plugin.

### Shell Aliases

Add these to your `.bashrc` or `.zshrc`:

```bash
alias a2h='alchemy convert -o hex'
alias a2b64='alchemy convert -o base64'
alias ahash='alchemy hash sha256'
```

## Development

### Building from Source

```bash
# Clone the repository
git clone https://github.com/rubenduburck/alchemy
cd alchemy

# Build the CLI tool
cargo build --release

# The binary will be at target/release/alchemy
```

### Build Requirements

This project uses the `rug` library for arbitrary precision arithmetic, which requires:
- GMP (GNU Multiple Precision Arithmetic Library)  
- MPFR (Multiple Precision Floating-Point Reliable Library)
- C compiler with GNU17 support

**Important**: The project is configured to compile with `-std=gnu17` to ensure compatibility with the rug dependency.

#### Installing Dependencies

**Ubuntu/Debian:**
```bash
sudo apt-get install libgmp-dev libmpfr-dev libmpc-dev
```

**macOS:**
```bash
brew install gmp mpfr
```

**Windows:**
Not supported due to GMP/MPFR dependency requirements.

### Cross-compilation

To build for different architectures:

```bash
# Build for specific target
make build-target TARGET=aarch64-unknown-linux-gnu

# Available targets:
# - x86_64-unknown-linux-gnu
# - aarch64-unknown-linux-gnu
# - x86_64-apple-darwin
# - aarch64-apple-darwin
```

### Supported Architectures

Pre-built binaries are available for:
- **Linux**: x86_64, aarch64
- **macOS**: x86_64 (Intel), aarch64 (Apple Silicon)

**Windows is not supported** due to the `rug` dependency requiring GMP/MPFR libraries which are difficult to build on Windows with MSVC.

### Running Tests

```bash
cargo test
```

## License

MIT