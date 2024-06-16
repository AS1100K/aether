# Auto-Mine Plugin

This plugin is the implementation of holding left-click and mines the block it is looking at.

## How to use this Plugin

Refer to the [example](../../examples/stone-miner) for better information on using this plugin.

Add this line to `cargo.toml`
```toml
[dependencies]
azalea-auto-mine = { git = "https://github.com/AS1100K/aether" }
```

Now, in `main.rs`
```rust
#[tokio::main]
async fn main() {
    ClientBuilder::new()
        .set_handler(handle)
        .add_plugins(AutoMinePlugin)
        .start(account, "10.9.12.3")
        .await
        .unwrap();
}
```

To enable `anti-afk` add the following line:
```rust
    bot.auto_mine(true);
```