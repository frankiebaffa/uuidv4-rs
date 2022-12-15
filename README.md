# uuidv4-rs

Generates a UUIDV4. That's it.

## Usage

Importing:

```toml
[dependencies.uuidv4-rs]
git = "https://github.com/frankiebaffa/uuidv4-rs"
tag = "v0.1.0"
```

or

```toml
uuidv4-rs = { git = "https://github.com/frankiebaffa/uuidv4-rs", tag = "v0.1.0" }
```

To obtain a `String`:

```rust
use uuidv4_rs::uuidv4;
fn main() {
    let uuid = uuidv4::<String>();
}
```

and to obtain an array of bytes (`[u8; 16]`):

```rust
use uuidv4_rs::uuidv4;
fn main() {
    let uuid = uuidv4::<[u8; 16]>();
}
```

If you wish to implement a new datatype for the uuid, all you
have to do is implement the `UUID` trait:

```rust
use uuidv4_rs::UUID;
impl UUID for SomeOtherType {
    fn from_uuid_bytes(bytes: [u8; 16]) -> Self {
        // ... handle the bytes
    }
}
```
