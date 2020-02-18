@echo off
rustup default nightly
xargo install --path . --force --target x86_64-pc-windows-msvc
rustup default stable
