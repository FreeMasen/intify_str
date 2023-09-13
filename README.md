# Intify Str

Compile time parsing of integer strings in rust.

## Example

```rust
use intify_str::intify_str_unsigned;

const VERSION_MAJOR: u64 = intify_str_unsigned!(env!("CARGO_PKG_VERSION_MAJOR"));
const VERSION_MINOR: u64 = intify_str_unsigned!(env!("CARGO_PKG_VERSION_MINOR"));
const VERSION_PATCH: u64 = intify_str_unsigned!(env!("CARGO_PKG_VERSION_PATCH"));
const VERSION_JOINED: u64 = (VERSION_MAJOR & 0xFF) << 24 | (VERSION_MINOR & 0xFF) << 16 | VERSION_PATCH & 0xFFFF;
fn main() {
    println!("THESE ARE INTS! {VERSION_MAJOR} {VERSION_MINOR} {VERSION_PATCH} {VERSION_JOINED}");
}
```
