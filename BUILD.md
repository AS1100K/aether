# Install Zig
```bash
brew install zig
cargo install cargo-zigbuild
rustup target add aarch64-unknown-linux-gnu
```

# Build
```shell
cargo zigbuild --release --target aarch64-unknown-linux-gnu
```