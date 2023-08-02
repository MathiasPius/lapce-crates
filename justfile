PLUGIN_DIR := "~/.local/share/lapce-stable/plugins/mathiaspius.lapce-crates"

build:
    cargo build

install: build
    rm -rf {{PLUGIN_DIR}}
    mkdir -p {{PLUGIN_DIR}}/bin
    cp volt.toml {{PLUGIN_DIR}}
    cp README.md {{PLUGIN_DIR}}
    cp target/wasm32-wasi/debug/lapce-crates.wasm {{PLUGIN_DIR}}/bin
