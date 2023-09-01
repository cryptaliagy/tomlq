# tq

This is a fork of [jamesmunns/tomlq](https://github.com/jamesmunns/tomlq), which updates the dependencies being used and adds some small quality of life elements, including unit tests, build/release pipelines, and automated deployment to Github/crates.io.

I wanted to fork this project since the original hasn't been updated since 2017 and I wanted to make sure that I had an easy way to consume it through binstall to use its functionality in CI without having to add the compile time of the tool to my pipelines. I also wanted to make sure that it was cleaned up of any security notices from the dependencies/std.

I've enabled dependabot scanning as well as weekly dependency auditing (using `cargo audit`) to help keep dependencies up-to-date moving forward.

## Contributions

Contributions are welcome! I just needed this one feature, so I haven't spent any additional time on this. If you have an idea, please feel free to open an issue, if you've implemented some additional features, please feel free to open a PR!

## License

Licensed under the [MIT License](./LICENSE)
