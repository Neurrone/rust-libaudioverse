# Rust-Libaudioverse

Rust bindings for [Libaudioverse][libaudioverse], a highly flexible realtime audio synthesis library.

## Documentation

[crates.io documentation](https://docs.rs/libaudioverse/).

## Requirements

* Llatest stable Rust.
* Libaudioverse binaries. The easiest way to obtain these is through the automated CI builds from the [main Libaudioverse repository][libaudioverse]. If on Windows using MSVC and Rustup,
    1. Copy `libaudioverse.lib` into `C:\Users\\{Your Username}\\.multirust\toolchains\\{current toolchain}\lib\rustlib\\{current toolchain}\lib`, where current toolchain is likely `stable-x86_64-pc-windows-msvc`.
    2. Copy `libaudioverse.dll` and `libsndfile-1.dll` into the same directory as your binary.

## Installation

From [crates.io][crates]:

```toml
    [dependencies]
    libaudioverse = "0.1"
```

Alternatively, pull from GitHub to obtain the latest development version:

```toml
    [dependencies.libaudioverse]
    git = "https://github.com/Neurrone/rust-libaudioverse"
```

## Generating libaudioverse-sys with bindgen

The optional feature "use-bindgen" generates bindings using the included libaudioverse headers, which requires [bindgen](https://github.com/rust-lang-nursery/rust-bindgen) to be installed. Generating this shouldn't be necessary in most cases.

[crates]: https://crates.io/
[libaudioverse]: https://github.com/libaudioverse/libaudioverse