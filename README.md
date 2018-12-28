[![Crates.io](https://img.shields.io/crates/v/r2ch.svg)](https://crates.io/crates/r2ch)
[![Docs](https://docs.rs/r2ch/badge.svg)](https://docs.rs/crate/r2ch/)
# r2ch - a Rust client for The 2ch Web API

## Disclaimer

- now in development

## Tasks

- [ ] Add docs
- [ ] Сaptcha
- [x] JSON impl (partly)
- [ ] Add much more examples
- [ ] Refactor

## Examples

Building examples.

`$ cd r2ch`

then run

`$ cargo  run --example boards_all`

## How to use

```rust
use r2ch::client::TwoCH;

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