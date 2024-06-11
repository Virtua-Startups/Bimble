#!/bin/bash

# Set up directories
mkdir -p bin/lin
mkdir -p bin/win

# Build bimble for Linux
cargo build --target x86_64-unknown-linux-gnu --release
if [ -f "./target/x86_64-unknown-linux-gnu/release/bimble" ]; then
    cp ./target/x86_64-unknown-linux-gnu/release/bimble ./bin/lin/bimble
else
    echo "Linux build for bimble failed."
fi

# Build bimble for Windows
cargo build --target x86_64-pc-windows-gnu --release
if [ -f "./target/x86_64-pc-windows-gnu/release/bimble.exe" ]; then
    cp ./target/x86_64-pc-windows-gnu/release/bimble.exe ./bin/win/bimble_win.exe
else
    echo "Windows build for bimble failed."
fi

# Move to bvm project
cd ../bvm

# Set up directories
mkdir -p bin/lin
mkdir -p bin/win

# Build bvm for Linux
cargo build --target x86_64-unknown-linux-gnu --release
if [ -f "./target/x86_64-unknown-linux-gnu/release/bvm" ]; then
    cp ./target/x86_64-unknown-linux-gnu/release/bvm ./bin/lin/bvm
else
    echo "Linux build for bvm failed."
fi

# Build bvm for Windows
cargo build --target x86_64-pc-windows-gnu --release
if [ -f "./target/x86_64-pc-windows-gnu/release/bvm.exe" ]; then
    cp ./target/x86_64-pc-windows-gnu/release/bvm.exe ./bin/win/bvm.exe
else
    echo "Windows build for bvm failed."
fi

# Copy built binaries to bimble project
if [ -f "./bin/win/bvm.exe" ]; then
    cp ./bin/win/bvm.exe ../bimble/bin/win/bvm.exe
else
    echo "Failed to copy Windows bvm.exe to bimble project."
fi

if [ -f "./bin/lin/bvm" ]; then
    cp ./bin/lin/bvm ../bimble/bin/lin/bvm
else
    echo "Failed to copy Linux bvm to bimble project."
fi
