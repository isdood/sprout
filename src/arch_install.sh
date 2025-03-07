#!/bin/bash

# Update the system
sudo pacman -Syu

# Install dependencies
sudo pacman -S --needed base-devel git

# Clone the repository
git clone https://github.com/isdood/sprout.git
cd sprout

# Build the project
cargo build --release

# Install the browser
sudo cp target/release/sprout /usr/local/bin/sprout

echo "Sprout browser has been installed successfully!"
