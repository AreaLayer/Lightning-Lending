# Lightning Lending :zap:

 Buy/Sell/Borrow/Lending on Lightning Network 
 
## How works

Lending channels direct on Lightning Network

Channel A opens with B with DLCs and Relays

Define HTLC, CLTV time with DLCs

After pay in sats

The channel is open

## Run software

**Pre Resqusite**

- Cargo

To build your CLI application, navigate to the project directory in the terminal and run:

```cargo
cargo build
```

To run your CLI application
```cargo
cargo run -- <main.rs> <cli.rs>
```
Replace PUBKEY_HEX, KIND, CONTENT, TAG1,TAG2, and SECRET_KEY_HEX with your actual data.
```cargo
cargo run -- -p PUBKEY_HEX -k KIND -c CONTENT -t TAG1,TAG2 -s SECRET_KEY_HEX
```
## Roadmap

- [X] Testnet/Signet
- [x] Integration with LDK and  Nostr
- [x] [PoC](https://github.com/AreaLayer/Lightning-lending-PoC/)
- [ ] Mainnet
- [ ] CLI
