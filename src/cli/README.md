# LL CLI Documentation

## Overview

The Event Generator CLI is a command-line tool that allows users to generate events by providing necessary parameters such as public key, event kind, content, tags, and secret key. It leverages Nostr, DLCs, and the Lightning Network for open/close channels and making payments.

### Features

- Open/close channels using Nostr.
- Make payments via CLI.
- Create and sign events with given parameters.

## Dependencies

Ensure you have the following dependencies in your `Cargo.toml`:

```toml
[dependencies]
clap = "2.33"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
sha2 = "0.9"
secp256k1 = "0.21"
hex = "0.4"
nostr = { git = "https://github.com/fiatjaf/nostr.git" }
rbf = "0.2"
```

## Usage

To use the Event Generator CLI, follow these instructions:

### Command Line Arguments

- `--pubkey, -p`: Public key in hexadecimal format (required)
- `--kind, -k`: Event kind (integer between 0 and 65535) (required)
- `--content, -c`: Event content (required)
- `--tags, -t`: Event tags, comma-separated (required)
- `--secret-key, -s`: Secret key in hexadecimal format (required)

### Example Command

```sh
event_generator_cli -p <PUBKEY_HEX> -k <KIND> -c <CONTENT> -t <TAGS> -s <SECRET_KEY_HEX>
```

### Example Usage

```sh
event_generator_cli -p 0234abcd... -k 1 -c "Hello, world!" -t "tag1,tag2" -s abcdef1234...
```

## Code Explanation

### Main Function

The `main` function is responsible for parsing command line arguments using the `clap` crate and passing these arguments to the `create_event` function.

### `create_event` Function

The `create_event` function generates an event by:

1. Calculating the current timestamp.
2. Creating a `MyEvent` struct with the provided parameters and calculated ID and signature.
3. Returning the created event.

### `calculate_id` Function

The `calculate_id` function computes the event ID by:

1. Serializing the provided data into a JSON-like string.
2. Hashing the serialized string using SHA-256.
3. Encoding the resulting hash into a hexadecimal string.

### `sign_event` Function

The `sign_event` function signs the event data by:

1. Initializing the Secp256k1 context.
2. Parsing the provided secret key and public key.
3. Creating a message from the serialized event data.
4. Signing the message with the secret key.
5. Returning the compact serialized signature in hexadecimal format.

### Structs

#### Event Struct

The `Event` struct represents an event with the following fields:

- `id`: Event ID (calculated)
- `pubkey`: Public key of the event creator
- `created_at`: Timestamp of event creation
- `kind`: Event kind (integer)
- `tags`: Tags associated with the event
- `content`: Event content
- `sig`: Signature of the event

## Example

Here's a complete example of running the CLI:

```sh
event_generator_cli -p 0234abcd... -k 1 -c "Test event content" -t "test,example" -s abcdef1234...
```

Expected output:

```json
{
    "id": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    "pubkey": "0234abcd...",
    "created_at": 1609459200,
    "kind": 1,
    "tags": ["test", "example"],
    "content": "Test event content",
    "sig": "3045022100dff9d8e...2b"
}
```

This output represents the generated event with its calculated ID and signature.