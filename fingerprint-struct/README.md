# fingerprint-struct

This crate allows for the computation of cryptographic hashes or arbitrary data structures.

It provides a `Fingerprint` trait which represents a type whose hash can be computed. It's implemented by default for most common types from `std`, such as primitives like `u32` or `bool`, collections like `Vec` or `BTreeSet`, pointers like `Box` or `Rc` or specialized types like `IpAddress`. It also provides a derive macro which generates a `Fingerprint` implementation for any struct or enum.

It relies on traits from the `digest` crate, which means its compatible with all [hash implementations](https://github.com/RustCrypto/hashes) from the [Rust Crypto project](https://github.com/RustCrypto/).

Hashes are considered stable, changes to how a given data structure is hashed will cause a minor version bump. Note that making a change to your own type definitions might introduce hash collisions. To avoid this, you can include a version number in your data structures.

## Instalation

Add the following lines to `Cargo.toml`:

```toml
[dependencies]
fingerprint-struct = "0.1.0"
```

or run:

```sh
cargo add fingerprint-struct
```

## Examples

### Hashing a string

```rust
use blake2::Blake2b512;
use fingerprint_struct::fingerprint;
use hex::ToHex;

let hash = fingerprint::<Blake2b512>("Hello world!");
let hash: String = hash.encode_hex_upper();
println!("{hash}");
```

### Hashing a custom data structure

```rust
use blake2::Blake2b512;
use fingerprint_struct::{fingerprint, Fingerprint};
use hex::ToHex;

#[derive(Fingerprint, Default)]
struct Book {
    title: String,
    rating: f32,
    authors: Vec<String>
}

let book = Book::default();
let hash = fingerprint::<Blake2b512>(book);
let hash: String = hash.encode_hex_upper();
println!("{hash}");
```

## `no_std` support

This crate supports `no_std` environments. Simply disable the default `std` feature:

```toml
[dependencies]
fingerprint-struct = { version = "0.1.0", default-features = false, features = ["derive"] }
```

You can also optionally enable the `alloc` feature on targets that don't support `std` but support `alloc`:

```toml
[dependencies]
fingerprint-struct = { version = "0.1.0", default-features = false, features = ["alloc", "derive"] }
```
