cargo +nightly build --target wasm32-unknown-unknown
wasm-bindgen ./target/wasm32-unknown-unknown/debug/agb_web.wasm --out-dir ./scripts
del /q .\out\*.wasm
webpack
