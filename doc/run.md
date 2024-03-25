# Running the CLI Application

### Prerequisite

- Rust
- LND 
- LDK Node
- CLN (Soon)
- Eclair (Soon)
- Rust Nostr
- Nostr profile (Npub)
- Testnet
- Mainnet (Soon)

Before running the CLI application, ensure that you have Cargo installed on your system.

### Add LND, CLN or Eclair endpoint

Add into your application LND, Eclair or CLN endpoint via ENV

```env
LND_ENDPOINT =
ECLAIR_API_ENDPOINT =
CLN_API_ENDPOINT =
```

## Building the Application

1. Open your terminal.

2. Navigate to the project directory containing your CLI application's source code.

3. Run the following command to build your CLI application:

```bash
cargo build
```

This command will compile your application, ensuring that all dependencies are resolved and the binary is generated.

## Running the Application

Once you've successfully built your CLI application, you can run it using the following command:

```bash
cargo run -- <main.rs> <cli.rs>
```

Replace the placeholders `<main.rs>` and `<cli.rs>` with the actual values you want to use. Here's an example of how to replace the placeholders:

```bash
cargo run -- -p PUBKEY_HEX -k KIND -c CONTENT -t TAG1,TAG2 -s SECRET_KEY_HEX
```

In the example above, replace `PUBKEY_HEX`, `KIND`, `CONTENT`, `TAG1,TAG2`, and `SECRET_KEY_HEX` with the actual data you want to pass as command-line arguments to your CLI application.

Now you should be able to run your CLI application with the specified parameters.
