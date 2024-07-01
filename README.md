# tq

This repository was previously available under `cryptaliagy/tq-rs`, and by the crate [`tq-rs`](https://crates.io/crates/tq-rs).

The current iteration of this crate supports both the `tq` and `tomlq` binaries. Wherever possible, `tq` should be preferred, and `tomlq` will be removed from the crate for the `0.2.0` version (scheduled for January 1, 2025).

## Installing

- Using `cargo` (compiles from source): `cargo install tomlq`
- Using `cargo-binstall` (downloads from Github Releases page): `cargo binstall -y tomlq`
  - _NOTE_ Using `cargo-binstall`, only the `tq` binary will be available. The `tomlq` binary is not installed by `binstall`

## Migrating from `tq-rs` crate

The only step that is required to migrate from `tq-rs` to `tomlq` is to change the `cargo install` (or `cargo binstall`) step from referencing `tq-rs` to referencing `tomlq`. The binary has the same name, and the usage is the same.

## Usage

> N.B. Use `tq --help` after installing for more information

### Getting package version from `Cargo.toml`

```
tq -f Cargo.toml 'package.version'
```

### Exporting package version as job output in Github Actions

```yaml
jobs:
  get-version:
    runs-on: ubuntu-latest
    outputs:
      version: ${{ steps.get-version.outputs.version }}
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Install cargo-binstall
        run: curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

      - name: Install tq
        run: cargo binstall -y tomlq

      - id: get-version
        name: Output the current version of the project
        run: echo "version=$(tq -f Cargo.toml 'package.version')" >> "$GITHUB_OUTPUT"
```

> N.B. See [the release pipeline](./.github/workflows/release.yaml) file for how this job is used.

## Contributions

Contributions are welcome! I just needed this one feature, so I haven't spent any additional time on this. If you have an idea, please feel free to open an issue, if you've implemented some additional features, please feel free to open a PR!

## License

Licensed under the [MIT License](./LICENSE)
