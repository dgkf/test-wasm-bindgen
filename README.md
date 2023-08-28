# `test-wasm-bindgen`

A small reproduction of a bug I've been unable to crack. To illicit the bug:

Build with

```
cargo install wasm-pack
RUST_BACKTRACE=1 wasm-pack build --target web
```
```
[INFO]: 🎯  Checking for the Wasm target...
[INFO]: 🌀  Compiling to Wasm...
    Finished release [optimized] target(s) in 0.01s
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /home/doug/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasm-bindgen-cli-support-0.2.87/src/descriptor.rs:218:15
stack backtrace:
   0: rust_begin_unwind
             at /rustc/5680fa18feaa87f3ff04063800aec256c3d4b4be/library/std/src/panicking.rs:593:5
   1: core::panicking::panic_fmt
             at /rustc/5680fa18feaa87f3ff04063800aec256c3d4b4be/library/core/src/panicking.rs:67:14
   2: core::panicking::panic_bounds_check
             at /rustc/5680fa18feaa87f3ff04063800aec256c3d4b4be/library/core/src/panicking.rs:162:5
   3: wasm_bindgen_cli_support::descriptor::Descriptor::_decode
   4: wasm_bindgen_cli_support::descriptor::Function::decode
   5: wasm_bindgen_cli_support::descriptor::Descriptor::_decode
   6: wasm_bindgen_cli_support::descriptors::execute
   7: wasm_bindgen_cli_support::Bindgen::generate_output
   8: wasm_bindgen_cli_support::Bindgen::generate
   9: wasm_bindgen::main
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
Error: Running the wasm-bindgen CLI
Caused by: Running the wasm-bindgen CLI
Caused by: failed to execute `wasm-bindgen`: exited with exit status: 101
  full command: "/home/doug/.cargo/bin/wasm-bindgen" "/home/doug/Projects/Programming/test-wasm-binding/target/wasm32-unknown-unknown/release/test_wasm_bindgen.wasm" "--out-dir" "/home/doug/Projects/Programming/test-wasm-binding/pkg" "--typescript" "--target" "web"[INFO]: 🎯  Checking for the Wasm target...
[INFO]: 🌀  Compiling to Wasm...
    Finished release [optimized] target(s) in 0.01s
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /home/doug/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasm-bindgen-cli-support-0.2.87/src/descriptor.rs:218:15
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Error: Running the wasm-bindgen CLI
Caused by: Running the wasm-bindgen CLI
Caused by: failed to execute `wasm-bindgen`: exited with exit status: 101
  full command: "/home/doug/.cargo/bin/wasm-bindgen" "/home/doug/Projects/Programming/test-wasm-binding/target/wasm32-unknown-unknown/release/test_wasm_bindgen.wasm" "--out-dir" "/home/doug/Projects/Programming/test-wasm-binding/pkg" "--typescript" "--target" "web"
```

This seems to have popped up from time to time in `wasm-bindgen`'s history with,
seemingly resolved by matching dependency versions (
[#3441](https://github.com/rustwasm/wasm-bindgen/issues/3441), 
[#2776](https://github.com/rustwasm/wasm-bindgen/issues/2776), among others).

## Troubleshooting

* Build error only arises when `IntoWasmAbi`/`FromWasmAbi` are implemented for 
  a new type.
* As far as I can tell, dependency versions are all the same. I've tried many 
  of the older versions that resolved previous issues to no avail 
  (`0.2.78`, `0.2.79`, `0.2.84`).
* Attempted reinstalling `wasm-pack` with `wasm-pack --locked` to no avail
