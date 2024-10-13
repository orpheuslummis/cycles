FROM rust:latest

RUN apt-get update && apt-get install -y \
    pkg-config \
    libgtk-3-dev \
    libglib2.0-dev \
    libcairo2-dev \
    libpango1.0-dev \
    libatk1.0-dev \
    libgdk-pixbuf2.0-dev \
    libjavascriptcoregtk-4.1-dev \
    libsoup-3.0-dev \
    libwebkit2gtk-4.1-dev \
    libxdo-dev

WORKDIR /usr/src/myapp

# Copy only the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to trick Cargo into downloading dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Download dependencies
RUN cargo fetch

# Remove the dummy src directory
RUN rm -rf src

# Now copy the real source code (excluding what's in .dockerignore)
COPY . .

# Build the release version
RUN cargo build --release

# Make the binary accessible
RUN cp target/release/cycles /usr/local/bin/cycles-linux-release