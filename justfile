PLUGIN_DIR := "~/.local/share/lapce-stable/plugins/mathiaspius.lapce-crates"
NIGHTLY_PLUGIN_DIR := "~/.local/share/lapce-stable/plugins/mathiaspius.lapce-crates"
DEBUG_PLUGIN_DIR := "~/.local/share/lapce-debug/plugins/mathiaspius.lapce-crates"

build:
    cargo build

install: build
    rm -rf {{PLUGIN_DIR}}
    mkdir -p {{PLUGIN_DIR}}/bin
    cp volt.toml {{PLUGIN_DIR}}
    cp README.md {{PLUGIN_DIR}}
    cp logo.png {{PLUGIN_DIR}}
    cp target/wasm32-wasi/debug/lapce-crates.wasm {{PLUGIN_DIR}}/bin
    
install-nightly: build
    rm -rf {{NIGHTLY_PLUGIN_DIR}}
    mkdir -p {{NIGHTLY_PLUGIN_DIR}}/bin
    cp volt.toml {{NIGHTLY_PLUGIN_DIR}}
    cp README.md {{NIGHTLY_PLUGIN_DIR}}
    cp logo.png {{NIGHTLY_PLUGIN_DIR}}
    cp target/wasm32-wasi/debug/lapce-crates.wasm {{NIGHTLY_PLUGIN_DIR}}/bin

install-debug: build
    rm -rf {{DEBUG_PLUGIN_DIR}}
    mkdir -p {{DEBUG_PLUGIN_DIR}}/bin
    cp volt.toml {{DEBUG_PLUGIN_DIR}}
    cp README.md {{DEBUG_PLUGIN_DIR}}
    cp logo.png {{DEBUG_PLUGIN_DIR}}
    cp target/wasm32-wasi/debug/lapce-crates.wasm {{DEBUG_PLUGIN_DIR}}/bin
