# EchoMambo Client

A lightweight TCP client for **EchoMambo**, built with **Rust** and **Tokio**. It receives messages from multiple client at the same time, forwards them to an EchoMambo server, waits for the echoed response, and returns the response to the original client.

---

## Dependencies

### Requirements

This project requires the following Rust crates:

```toml
tokio = { version = "1", features = ["full"] }
```

---

## Overview

The project exposes a single asynchronous function:

- **`call_echo_mambo()`** – Connects to an [EchoMambo](https://github.com/PromiseJoseph/echo-mambo) server, sends a message, waits for the response, and returns it as a `String`.

The client communicates with an EchoMambo server running over TCP.

---

## Usage

### 1. Build the Project

```bash
cargo build --release
```

---

### 2. Start an EchoMambo Server

Before using the client, make sure the [EchoMambo](https://github.com/PromiseJoseph/echo-mambo) server is running.

By default, the client connects to `ECHOMAMBO_SERVER_ADDR` constant, which is set to:
`127.0.0.1:7777`

---

### Start the Client

```bash
cargo run --release
```

## Sserver will start listening on `127.0.0.1:7778`

### 3. Call the Client

- rust-tcp-echo
  You can visit [rust-tcp-echo](https://github.com/PromiseJoseph/rust-tcp-echo) to use the available echo client

- nc

```bash
nc 127.0.0.1 7778
```

---

## Configuration

The client server address is configured using the `ECHOMAMBO_CLIENT_ADDR` constant.

Default configuration:

```text
127.0.0.1:7778
```

---

## Example

Request:

```text
Hello, EchoMambo!
```

Response if any:

```text
EchoMambo responded: Hello, EchoMambo!
```

---

## Notes

- Uses asynchronous networking powered by Tokio.
- Uses a fixed receive buffer of **1024 bytes**.
- Assumes an EchoMambo server is already running and reachable.

---

## Roadmap

Future improvements include:

- Configurable server address via CLI or environment variables
- Better error handling using `Result`
- TLS support
- Connection timeout support
- Automatic reconnection

---

## License

This project is licensed under the **MIT License**. See the [LICENSE]() file for details.
