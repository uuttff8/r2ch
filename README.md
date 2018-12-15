# r2ch - a Rust client for The 2ch Web API

## Disclaimer

- now in development

## Tasks

- [x] Publish to [crates.io](https://crates.io)
- [ ] Add docs
- [ ] Сaptcha
- [ ] Add much more examples
- [ ] Refactor

## Examples

Building examples.

`$ cd r2ch`

then run

`$ cargo  run --example boards_all`

## How to use

```rust
let _ = TwoCH::default().boards_all();
```

### API Documentation

Not yet
> you may find docs in sources

### CHANGELOG

Please see the [CHANGELOG](./CHANGELOG.md) for a release history.

### Contribution

If you find any problem or have suggestions about this crate, please submit an
issue. Moreover, any pull request ,code review and feedback are welcome.

### License

[MIT](./LICENSE)