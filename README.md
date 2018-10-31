# Rusty Elm???

This is an experiment

```
cd examples/counter/

# See https://github.com/rustwasm/wasm-pack/issues/252
ln -s ../../target target
cargo install wasm-pack
wasm-pack build

cd www/
yarn
yarn start
```
