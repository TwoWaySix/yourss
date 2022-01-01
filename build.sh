#!/bin/bash
export YOURSS_IP="127.0.0.1"
export YOURSS_FILESERVER_PORT="8880"
export YOURSS_FEEDBUILDER_PORT="8881"
export YOURSS_DOWNLOADER_PORT="8882"
export YOURSS_FRONTEND_PORT="8883"

export YOURSS_FILESERVER="$YOURSS_IP:$YOURSS_FILESERVER_PORT"
export YOURSS_FEEDBUILDER="$YOURSS_IP:$YOURSS_FEEDBUILDER_PORT"
export YOURSS_DOWNLOADER="$YOURSS_IP:$YOURSS_DOWNLOADER_PORT"
export YOURSS_FRONTEND="$YOURSS_IP:$YOURSS_FRONTEND_PORT"

# Creating directories
mkdir -p build/static
mkdir build/static/root
mkdir build/static/mp3
mkdir build/static/rss
mkdir build/static/images

# Compiling and copying yourss-feedbuilder
cargo build --release --bin feedbuilder
cp ./target/release/feedbuilder ./build/

# Compiling and copying yourss-downloader
cargo build --release --bin downloader
cp ./target/release/downloader ./build/

# Compiling and copying yourss-fileserver
cargo build --release --bin fileserver
cp ./target/release/fileserver ./build/

# Compiling and copying yourss-frontend
cargo build --release --bin frontend
cp ./target/release/frontend ./build/

