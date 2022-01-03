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

# Building the binaries
cargo build --release

# Copying the binaries
cp ./target/release/feedbuilder ./build/yourss-feedbuilder
cp ./target/release/downloader ./build/yourss-downloader
cp ./target/release/fileserver ./build/yourss-fileserver
cp ./target/release/frontend ./build/yourss-frontend

# Copying the run script
cp ./templates/start_yourss.sh ./build/

# TODO: Dockerize the result!

