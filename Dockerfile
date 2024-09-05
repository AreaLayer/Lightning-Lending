# Base image for building
FROM rust:1.80.1 as builder

# Install dependencies and tools
RUN apt-get update && apt-get install -y \
    git \
    dpkg \
    && rm -rf /var/lib/apt/lists/*

# Build BDK
RUN git clone https://github.com/bitcoindevkit/bdk.git /bdk
WORKDIR /bdk
RUN cargo build --release

# Build Nostr
RUN git clone https://github.com/rust-nostr/nostr.git /nostr
WORKDIR /nostr
RUN cargo build --release

# Build DLC
RUN git clone https://github.com/p2pderivaives/rust-dlc.git /dlc
WORKDIR /dlc
RUN cargo build --release

# Build Lightning
RUN git clone https://github.com/lightningdevkit/rust-lightning.git /lightning
WORKDIR /lightning
RUN cargo build --release

# Install Lightning-Lending project
RUN git clone https://github.com/Arealayer/Lightning-Lending.git /Lightning-Lending
WORKDIR /Lightning-Lending

# Install cargo-deb to create .deb package
RUN cargo install cargo-deb

# Build the .deb package
RUN cargo deb

# Install the Lightning-Lending .deb package
RUN dpkg -i /Lightning-Lending/target/debian/lightning-lending_0.1.0_amd64.deb

# Final image
FROM debian:bullseye-slim

# Copy the compiled binaries from the builder stage
COPY --from=builder /bdk/target/release/bdk /usr/local/bin/bdk
COPY --from=builder /nostr/target/release/nostr /usr/local/bin/nostr
COPY --from=builder /dlc/target/release/dlc /usr/local/bin/dlc
COPY --from=builder /lightning/target/release/lightning /usr/local/bin/lightning
COPY --from=builder /Lightning-Lending/target/debian/lightning-lending_0.1.0_amd64.deb /usr/local/bin/lightning-lending

# Set default command (you can change this as per your requirements)
CMD ["lightning-lending"]
