#!/bin/bash

# Creating directories and copying files
mkdir build
mkdir build/static
mkdir build/static/root
mkdir build/static/mp3
mkdir build/static/rss
mkdir build/static/images
cp static/index.html build/static/
cp -a static/css build/static/
cp -a static/js build/static/
cp -a static/images build/static/
cp -a templates build/

# Compiling and copying yourss-feedbuilder
cargo build --bin feedbuilder
cp ./target/debug/feedbuilder ./build/yourss-feedbuilder

# Compiling and copying yourss-downloader
cargo build --bin downloader
cp ./target/debug/downloader ./build/yourss-downloader

# Compiling and copying yourss-fileserver
cargo build --bin fileserver
cp ./target/debug/fileserver ./build/yourss-fileserver

# Compiling and copying yourss-frontend
cargo build --bin frontend
cp ./target/debug/frontend ./build/yourss-frontend

# Copying the run script
cp ./templates/start_yourss.sh ./build/

# TODO: Dockerize the result!

