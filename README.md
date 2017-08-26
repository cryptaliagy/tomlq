# tomlq

I wanted a command line tool like `jq` for TOML. Couldn't find one, so I'm making one.

Right now you can only do one thing: Get a value from a toml file.

Given the `Cargo.toml` in this repo:

```bash
➜  tomlq git:(master) ✗ tomlq dependencies.toml -f Cargo.toml
0.4
➜  tomlq git:(master) ✗ echo $?
0
➜
➜  tomlq git:(master) ✗ tomlq dependencies.toml.foo -f Cargo.toml
dependencies.toml.foo not found!
➜  tomlq git:(master) ✗ echo $?
255
➜  tomlq git:(master) ✗
```

## License

Licensed under the [MIT License](./LICENSE)