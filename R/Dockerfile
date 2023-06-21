# Get R base image
FROM r-base

# Install dependencies
RUN 	apt-get update &&\
	apt-get install -y \
	git &&\
	rm -rf /var/lib/apt/lists/* &&\
	R -e 'install.packages(c("bnlearn", "microbenchmark"))'

# Clone repository and set commit
RUN git clone https://github.com/AlessioZanga/causal-hub.git
WORKDIR /causal-hub
RUN git checkout -b 82-add-pc-stable-algorithm 9c8b281 &&\
	unzip -o tests/assets -d tests
