#!/bin/bash

# Build the Docker image
docker build -t rust-linux-build .

# Copy the binary from the container
docker create --name temp_container rust-linux-build
docker cp temp_container:/usr/local/bin/cycles-linux-release ./cycles-linux-release
docker rm temp_container

# Check if the release binary was created and copied
if [ -f "cycles-linux-release" ]; then
    echo "Release build successful. Linux binary is now in the current directory as 'cycles-linux-release'"
    # Make the binary executable
    chmod +x cycles-linux-release
else
    echo "Release build failed or binary not found."
fi