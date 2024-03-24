# How to contribute to the project

## Requirements
This crate uses `rustc_middle`, which is very nightly-specific. In order to compile this crate, you need to have installed the nightly toolchain.

```bash
rustup toolchain install nightly # To install the nightly toolchain
rustup default nightly # To set the nightly toolchain as the default
```

In order to be able to compile parts of core rustc, you need to have installed:

* `rust-src` component, which contains the source code of the standard library
* `rustc-dev` component, which contains the source code of the compiler
* `llvm-tools-preview` component, which contains the `llvm-tools` package

```bash
rustup component add rust-src rustc-dev llvm-tools-preview
```

Most likely you are using `rust-analyzer` as your IDE. At the time of writing, `rust-analyzer` has some issues with
extern crates. In order to avoid false positive errors, you need to open up the settings of your IDE and add to the ignored
errors `unresolved-extern-crate`. For instance, in VSCode, you can do this by opening up the settings (`Ctrl + ,`), searching for
`analyzer.diagnostics.disabled` and there you can add the error.