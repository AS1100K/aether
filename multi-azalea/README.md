# Multi Azalea

This library is created to support multiple versions of azalea using the power of cargo features.

## How to use
Just add the following line in your `Cargo.toml`
```toml
[dependencies]
azalea = { package = "multi-azalea", git = "https://github.com/as1100k/aether.git", feature = "git", default_features = false }
```

_For using any other feature other than `git` `default_features = false` is required._

## Supported Azalea Version

When creating this plugin, my aim was to support multiple versions of [`azalea`](https://github.com/azalea-rs/azalea.git)
which are supported with the help of cargo features.

| Feature           | Supported Version                                                              |
|-------------------|--------------------------------------------------------------------------------|
| `git` _(default)_ | `main` branch                                                                  |
| `crates_io`       | `0.9.1` version on [`crates.io`](https://crates.io/crates/azalea)              |
| `git_1_20_6`      | `1.20.6` branch of [`azalea`](https://github.com/azalea-rs/azalea/tree/1.20.6) |

It is recommended to use the bleeding-edge version of azalea _(tracking `main` branch)_ or `1.20.6` branch as these are
somewhat stable versions and since you are using zenith proxy, that uses ViaProxy behind the scenes you  can connect to
any server of any backward version.

> [!WARNING]
> `main` branch of azalea can introduce breaking changes especially in upgrading minecraft version.

## Best Practices

1. If you want to use your own fork, set up your `Cargo.toml` in the following way:
   ```toml
   [dependencies]
   azalea = { git = "https://github.com/azalea-rs/azalea.git", branch = "1.20.6" }
   
   # for more information see: https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html#overriding-repository-url
   [path."https://github.com/azalea-rs/azalea"]
   azalea = { git = "https://github.com/as1100k-forks/azalea.git", branch = "better-1.20.6" }
   ```
   
   This way you can use your fork with plugins/dependency that are on git version of azalea.
   
> [!NOTE]
> Using `patch` will only make you use your fork on your library and plugin/library will still be using
> whatever version of azalea they are using.

2. If you are building a library/plugin and want to support multiple versions of azalea, then you should set up your 
   `Cargo.toml` in the following way:
   
   _let's assume that you want to support `main` branch and `1.20.6` branch._
   ```toml
   [features]
   default = ["git"]
   git = ["azalea/git"]
   git_1_20_6 = ["azalea/git_1_20_6"]
   
   [dependencies]
   azalea = { package = "multi-azalea", git = "https://github.com/as1100k/aether.git", default_features = false }
   ```
   
   Now, it's upto the user which version they want to use.
