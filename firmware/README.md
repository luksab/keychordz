# Toolchain:
Because of a [bug](https://github.com/avr-rust/blink/issues/29), currently we must use a version like `nightly-2021-01-07`

```
cargo +nightly-2021-01-07 build -Z build-std=core --target=atmega32u4.json --release
```