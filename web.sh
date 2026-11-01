#!/bin/sh
# Web build via emscripten, per raylib-rs CI (.github/workflows/web.yml).
export PATH="/usr/lib/emscripten:$PATH"
# Required by raylib-sys/build.rs (panics without EMCC_CFLAGS for emscripten target).
# --embed-file: blip.wav is loaded from the virtual FS inside the wasm, no extra .data fetch.
export EMCC_CFLAGS="-O3 -sUSE_GLFW=3 -sASSERTIONS=1 -sWASM=1 -sASYNCIFY -sGL_ENABLE_GET_PROC_ADDRESS=1 -fexceptions -sFORCE_FILESYSTEM=1 --embed-file src/resources"
# bindgen runs host clang; point it at the emscripten sysroot instead of /usr/include.
export BINDGEN_EXTRA_CLANG_ARGS="--sysroot=$HOME/.cache/emscripten/sysroot --target=wasm32-emscripten"
RUSTFLAGS='-C linker=em++' cargo build --release --target wasm32-unknown-emscripten || exit 1
cp target/wasm32-unknown-emscripten/release/pong.js target/wasm32-unknown-emscripten/release/pong.wasm web/
echo "Play at: python3 -m http.server 8080 --directory web"
