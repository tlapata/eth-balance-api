# Ethereum Balance API (Rust + Axum)

A simple Rust microservice that returns the ETH balance of a given address using `ethers-rs` and `axum`.

## Features

- Connects to Ethereum via RPC
- Parses address from query
- Returns balance in ETH
- Async performance via Tokio

## Usage

Set your RPC URL first:

```bash
export RPC_URL=https://mainnet.infura.io/v3/YOUR_KEY
```

### 2. Run the server

```bash
cargo run
```

### 3. Query balance

```http
GET /balance?address=0x742d35Cc6634C0532925a3b844Bc454e4438f44e
```

Example:
```
http://localhost:3000/balance?address=0x742d35Cc6634C0532925a3b844Bc454e4438f44e
```

## Dependencies

- [Axum](https://docs.rs/axum)
- [Ethers-rs](https://docs.rs/ethers)
- [Tokio](https://tokio.rs/)

## License

MIT
