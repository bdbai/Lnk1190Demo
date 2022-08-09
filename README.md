# Lnk1190Demo
Repro of https://developercommunity.visualstudio.com/t/fatal-error-LNK1190:-invalid-fixup-foun/10113862

## Build

1. Prepare `rustup` from https://rustup.rs/
2. Build the Rust package:

```sh
cd staticrustlib
cargo build
```

rustup will install the toolchain version specified in `rust-toolchain.toml`.

3. Open `Lnk1190Demo` in Visual Stdio 2022 and build `Debug|x64` configuration.

