# create_rounded_corners_image
Simple library that is used in my telegram bot.

Implemented with FFI ( npm ffi-napi ) for further usage in node.js environment.

## How to use
Firstly build:
```bash
cargo build --release # ALSO DEPENDS ON YOUR OS, LOOK DOCS
```
Then in `target/relese` you'll find out `libimg` files with different extensions( `.so` for linux ), so you may use them.

## Functions view
```rust
fn create_rounded_corners_image( inp: string, out: string, radius: int ): bool
```