# tq

This is a fork of [jamesmunns/tomlq](https://github.com/jamesmunns/tomlq), which updates the dependencies being used and adds some small quality of life elements, including unit tests, build/release pipelines, and automated deployment to Github/crates.io.

I wanted to fork this project since the original hasn't been updated since 2017 and I needed an easy, quick way to consume it in CI. I also wanted to make sure that it was cleaned up of any security notices from the dependencies/std.

I've enabled dependabot scanning as well as weekly dependency auditing (using `cargo audit`) to help keep dependencies up-to-date moving forward.

## Installing

- Using `cargo` (compiles from source): `cargo install tq-rs`
- Using `cargo-binstall` (downloads from Github Releases page): `cargo binstall -y tq-rs`

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
        uses: actions/checkout@v3

      - name: Install cargo-binstall
        run: curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

      - name: Install tq
        run: cargo binstall -y tq-rs

      - id: get-version
        name: Output the current version of the project
        run: echo "version=$(tq -f Cargo.toml 'package.version')" >> "$GITHUB_OUTPUT"
```

> N.B. See [the release pipeline](./.github/workflows/release.yaml) file for how this job is used.

## Contributions

Contributions are welcome! I just needed this one feature, so I haven't spent any additional time on this. If you have an idea, please feel free to open an issue, if you've implemented some additional features, please feel free to open a PR!

## License

Licensed under the [MIT License](./LICENSE)
