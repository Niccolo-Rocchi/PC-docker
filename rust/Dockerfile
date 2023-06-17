# Get base image
FROM debian
# Install dependencies
RUN apt-get update && apt-get install -y \
	curl \
	build-essential \
	libopenblas-dev \
	pkg-config \
	libssl-dev \
	graphviz \
	&& rm -rf /var/lib/apt/lists/*
# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
