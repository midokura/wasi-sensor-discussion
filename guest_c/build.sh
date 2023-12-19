#! /bin/sh
set -e
if [ ! -f wasi_snapshot_preview1.reactor.wasm ]; then
    curl --fail -L -O https://github.com/bytecodealliance/wasmtime/releases/download/v15.0.1/wasi_snapshot_preview1.reactor.wasm
fi

if [ ! -d libjpeg ]; then
    mkdir libjpeg
    cd libjpeg
    curl --fail -L https://github.com/yamt/libjpeg/releases/download/wasm32-wasi-20231128/libjpeg-wasm32-wasi.tgz | pax -rvz
    cd ..
fi

# prepare wasi-sdk. this is copy-and-paste from:
# https://github.com/yamt/toywasm/blob/master/build-wasm32-wasi.sh

MAJOR=${WASI_SDK_MAJOR:-21}
MINOR=${WASI_SDK_MINOR:-0}
WASI_SDK_DIR=${WASI_SDK_DIR:-$(pwd)/.wasi-sdk-${MAJOR}.${MINOR}}
DIST_DIR=.dist

mkdir -p ${DIST_DIR}

fetch_wasi_sdk()
{
    UNAME=$(uname -s)
    case ${UNAME} in
    Darwin)
        PLATFORM=macos
        ;;
    Linux)
        PLATFORM=linux
        ;;
    *)
        echo "Unknown uname ${UNAME}"
        exit 1
        ;;
    esac
    TAR=wasi-sdk-${MAJOR}.${MINOR}-${PLATFORM}.tar.gz
    if [ ! -f ${DIST_DIR}/${TAR} ]; then
        URL=https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-${MAJOR}/${TAR}
        curl -L -o ${DIST_DIR}/${TAR} ${URL}
    fi
    pax -rz \
    -f ${DIST_DIR}/${TAR} \
    -s"!^wasi-sdk-${MAJOR}\.${MINOR}!${WASI_SDK_DIR}!"
}

test -d "${WASI_SDK_DIR}" || fetch_wasi_sdk

CC=${CC:-${WASI_SDK_DIR}/bin/clang}
wit-bindgen c ../wit
${CC} -Os -I./libjpeg/include -mexec-model=reactor -o guest.wasm main.c sensing.c sensing_component_type.o -L./libjpeg/lib -ljpeg
wasm-tools component new \
guest.wasm \
-o guest-component.wasm \
--adapt ./wasi_snapshot_preview1.reactor.wasm
WASMTIME_BACKTRACE_DETAILS=1 ${WASMTIME:-wasmtime} compile --wasm component-model guest-component.wasm
