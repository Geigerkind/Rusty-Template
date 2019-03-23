# Setting up rust
rustup default nightly

# Compiling the backend
echo "Compiling the backend...";
cd /me/Backend;
cargo build > /dev/null 2> /dev/null;
echo "Compilation done...";