FROM rust as builder
FROM  rust-bitcoin
FROM rust-nostr 

RUN cargo install --path.
RUN cargo install cargo-deb
RUN git clone https://github.com/Arealayer/Lightning-Lending.git
RUN cd Lightning-Lending && cargo deb
RUN dpkg -i Lightning-Lending/target/debian/lightning-lending_0.1.0_amd64.deb


