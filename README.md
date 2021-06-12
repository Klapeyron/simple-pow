# simple-pow

Finds 4-byte prefix, that SHA-256 sum of prefix combined with input hex encoded bytes ends with [0xca, 0xfe]

## Usage

```
cargo run --release -- <hex_encoded 64-byte long string>
```

or

```
simple-pow <hex_encoded 64-byte long string>
```

## Example
```
cargo test specification_example_test
```