# Get Rust base image
FROM rust:1.70.0-buster

# Install dependencies
RUN 	apt-get update &&\
	apt-get install -y \
	git \
	build-essential \
	libopenblas-dev \
	pkg-config \
	libssl-dev \
	graphviz \
	gnuplot &&\
	rm -rf /var/lib/apt/lists/*

# Clone repository and set commit
RUN git clone https://github.com/AlessioZanga/causal-hub.git
WORKDIR /causal-hub
RUN git checkout -b 82-add-pc-stable-algorithm 9c8b281 &&\
	unzip -o tests/assets -d tests
