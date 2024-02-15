#! /bin/sh
set -e
if [ ! -f wasi_snapshot_preview1.reactor.wasm ]; then
    curl --fail -L -O https://github.com/bytecodealliance/wasmtime/releases/download/v17.0.0/wasi_snapshot_preview1.reactor.wasm
fi

release_opt=--release
release=release

#release_opt=
#release=debug

cargo build ${release_opt} --target wasm32-wasi
wasm-tools component new \
./target/wasm32-wasi/${release}/guest.wasm \
-o guest-component.wasm \
--adapt ./wasi_snapshot_preview1.reactor.wasm
WASMTIME_BACKTRACE_DETAILS=1 ${WASMTIME:-wasmtime} compile --wasm component-model guest-component.wasm
