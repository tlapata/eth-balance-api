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
