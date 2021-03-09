
### Functions export example

```
cargo-fmt && cargo build --target wasm32-unknown-unknown --release
wasm2wat ./target/wasm32-unknown-unknown/release/rust_secp256k1.wasm | less
```
