#!/bin/bash

# Update the system
sudo apt-get update

# Install dependencies
sudo apt-get install -y curl git

# Download and install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# Clone the repository
git clone https://github.com/isdood/sprout.git
cd sprout

# Build the project
cargo build --release

# Install the browser
sudo cp target/release/sprout /usr/local/bin/sprout

echo "Sprout browser has been installed successfully!"
