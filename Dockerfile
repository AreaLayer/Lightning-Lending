FROM rust:1.80.0 as builder
FROM bdk:1.0.0-beta.1 as bdk
FROM nostr:0.30.0 as nostr
FROM dlc:0.5.0 as dlc
FROM lightning:0.0.123 as lightning

RUN cargo install --path.
RUN cargo install cargo-deb
RUN git clone https://github.com/Arealayer/Lightning-Lending.git
RUN cd Lightning-Lending && cargo deb
RUN dpkg -i Lightning-Lending/target/debian/lightning-lending_0.1.0_amd64.deb


