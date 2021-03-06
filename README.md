# angsd-io

[![GitHub Actions status](https://github.com/malthesr/angsd-io/workflows/CI/badge.svg)](https://github.com/malthesr/angsd-io/actions)

**angsd-io** is a Rust crate for reading and writing common formats from the [ANGSD](https://github.com/ANGSD/angsd) suite of bioinformatics tools.

## Usage

**angsd-io** is not yet published on [crates.io](https://crates.io/), as the API is still in flux.

To use **angsd-io** in your own project, you can depend on this github repo. To do so, add the following to the `[dependencies]` section of your `Cargo.toml`:

```
angsd-io = { git = "https://github.com/malthesr/angsd-io.git" }
```

For more information, including on how to depend on a particular commit, see [here](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories).

By default, **angsd-io** depends on the [ndarray](https://github.com/rust-ndarray/ndarray) crate to conveniently read to and from array structures. If you do not need this, you may wish to disable this dependency:

```
angsd-io = { git = "https://github.com/malthesr/angsd-io.git", default-features = false }
```

## Examples

The [`examples`](examples/) sub-directory contains runnable examples of illustrative basic usage of the crate. For better performance, compile these with the `--release` flag. For instance, to read a SAF file and print it to stdout,

```
cargo run --release --example read_saf [PATH_TO_SAF]
```

To read the intersecting sites of multiple SAF files,

```
cargo run --release --example merge_saf [PATHS_TO_SAFS] 
```

## Documentation

The documentation can be built and viewed locally by running

```
cargo doc --open
```
