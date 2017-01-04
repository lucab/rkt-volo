# rkt-volo: a rust stage1-fly experiment

[![](https://tokei.rs/b1/github/lucab/rkt-volo?category=code)](https://github.com/lucab/rkt-volo)

rkt-volo is an experimental stage1 for [rkt](https://github.com/coreos/rkt), written in [Rust](https://www.rust-lang.org).

It is supposed to implement the same features of rkt [stage1-fly](https://coreos.com/rkt/docs/latest/running-fly-stage1.html) (single-app pod, no isolation).

Disclaimer: `Experimental only, not intended for production usage`.

## Build requirements

Compiling stage1-volo requires some tools installed on the host:
 * rustc & cargo (`x86_64-unknown-linux-musl` target)
 * [acbuild-script](https://github.com/containers/build)

The quickest way to get a rust musl cross-toolchain is [rustup](https://github.com/rust-lang-nursery/rustup.rs#installation):

```
rustup target add x86_64-unknown-linux-musl
```

## Building stage1 images

Cross-compile volo stage1 entrypoints (initial build may take a while):

```
(cd stage1/volo/; cargo build --release --target x86_64-unknown-linux-musl -p rkt-stage1-volo)
```

Then pack and sign the rkt-stage1 image:

```
./dist/stage1-volo.acbuild
```

This should result in a working rkt-stage1 image, available under `target/stage1-volo.aci`, which can be locally fetched:
```
rkt fetch --insecure-options=image target/stage1-volo.aci
```

## Pre-built images

Development snapshots are built on a non-regular base and made available on Github for testing purposes.

They can be fetched via appc discovery:
```
rkt fetch --trust-keys-from-https aci.lucabruno.net/rkt/stage1-volo
```
