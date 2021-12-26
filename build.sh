# Creating directories
mkdir -p build/static
mkdir build/static/root
mkdir build/static/mp3
mkdir build/static/rss
mkdir build/static/images

# Compiling and copying feedbuilder
cd feedbuilder
cargo build --release
cp ./target/release/yourss-feedbuilder ../build/
cd ..

# Compiling and copying fileserver
cd fileserver
cargo build --release
cp ./target/release/yourss-fileserver ../build/

