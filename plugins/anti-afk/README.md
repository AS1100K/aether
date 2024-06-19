# Anti-AFK Plugin

This plugin does certain actions like random head movement, swing and flip the nearest lever to not get AFK kicked
every 3 second.

## How to use this Plugin
Refer to the [example](../../examples/anti-afk) for better information on using this plugin.

Add this line to `cargo.toml`
```toml
[dependencies]
azalea-anti-afk = { git = "https://github.com/AS1100K/aether" }
```

Now, in `main.rs`
```rust
#[tokio::main]
async fn main() {
    ClientBuilder::new()
        .set_handler(handle)
        .add_plugins(AntiAFKPlugin)
        .start(account, "10.9.12.3")
        .await
        .unwrap();
}
```

To enable `anti-afk` add the following line:
```rust
    bot.set_anti_afk(true);
```