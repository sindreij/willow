# Willow

**Willow** is an experiment to see if it is possible to create a "elm-like" API using Rust.

## Demo / Examples

- Counter [source code](examples/counter/src/app.rs). [Demo](http://sindrejohansen.no/willow/counter/)
- TodoMVC [source code](examples/todomvc/src/app.rs). [Demo](http://sindrejohansen.no/willow/todomvc/)

## Quickstart

```sh
cd examples/counter/

# See https://github.com/rustwasm/wasm-pack/issues/252
ln -s ../../target target
cargo install wasm-pack
wasm-pack build

cd www/
yarn
yarn start
```

## Backstory

[Se my blogpost for the why and how I built this.](https://sindrejohansen.no/blog/willow/rust/elm/2018/11/14/willow-elm-in-rust.html)

## License

Licensed under either of these:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <https://opensource.org/licenses/MIT>)

### Contributing

Unless you explicitly state otherwise, any contribution you intentionally submit
for inclusion in the work, as defined in the Apache-2.0 license, shall be
dual-licensed as above, without any additional terms or conditions.
