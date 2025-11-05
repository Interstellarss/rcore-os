# rosh

A teaching operating system kernel written in Rust, developed by following the [Writing an OS in Rust](https://os.phil-opp.com/) tutorial. The repository demonstrates how to use a custom target specification, integrate the `bootloader` crate, and run the built image in QEMU through `bootimage`.


The repository includes a custom `osconfig.json` target file and augments `.cargo/config.toml` to make the compiler use it, while enabling `build-std` support and configuring the runner:

```toml
[build]
target = "osconfig.json"

[unstable]
build-std = ["core", "compiler_builtins"]
build-std-features = ["compiler-builtins-mem"]

[target.'cfg(target_os = "none")']
runner = "bootimage runner"
```

## Toolchain Installation

```bash
rustup component add rust-src llvm-tools-preview --toolchain nightly-x86_64-pc-windows-msvc
rustup override set nightly
```

## Bootloader Support

Install the required tooling and declare the dependency:

```bash
cargo install bootimage
rustup component add llvm-tools-preview
```

```toml
[dependencies]
bootloader = "0.9"
```

## Build

Compile with the nightly toolchain using the custom target:

```bash
cargo +nightly build --target osconfig.json
```

The compiled binary is placed at `target/osconfig/debug/hello`.

Create a bootable disk image:

```bash
cargo bootimage
```

## Run in QEMU

With QEMU installed, start the kernel with the default runner:

```bash
cargo run
```

`cargo run` invokes the `bootimage runner`, which launches QEMU with the generated image.

## Reference

- Writing an OS in Rust — https://os.phil-opp.com/
