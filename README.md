# 🚀 SolanaTradeX-Rust

A high-performance **trading client for Solana** written in Rust.  
Built with **BlockRazor relay integration**, this client enables developers to send transactions via **gRPC** and **HTTP** with support for **fast mode** and **sandwichMitigation mode**.  

---

## 📖 Documentation
Full documentation available at: [BlockRazor Docs](https://blockrazor.xyz/docs)

---

## ⚡ Quickstart

### 1. Clone the Repository
```bash
git clone https://github.com/BlockRazorinc/solana-tradex-rust.git
cd solana-tradex-rust
````

### 2. Install Dependencies

* Install [protoc](https://grpc.io/docs/protoc-installation/)
* Build the project:

```bash
cargo build
```

---

## 🔧 Configuration

Edit the source files before running examples:

### gRPC — Fast Mode

`src/mode_grpc_fast.rs`

```rust
let blzendpoint = "http://frankfurt.solana-grpc.blockrazor.xyz:80"; // Relay endpoint
let mainnetrpc = "";     // Your Solana RPC endpoint
let authkey = "";        // Your Auth Key
let privatekey = "";     // Your wallet private key (base58)
let mode = "";           // Send mode
let tipamount  = 1_000_000;
```

Run:

```bash
cargo run --bin mode-grpc-fast
```

---

### gRPC — Sandwich Mitigation

`src/mode_grpc_sandwichMitigation.rs`

```rust
let blzendpoint = "http://frankfurt.solana-grpc.blockrazor.xyz:80";
let mainnetrpc = "";
let authkey = "";
let privatekey = "";
let mode = "sandwichMitigation";
let safe_window = Some(5);
let revert_protection = false;
```

Run:

```bash
cargo run --bin mode-grpc-sandwichMitigation
```

---

### HTTP — Fast Mode

`src/mode_http_fast.rs`

```rust
let http_endpoint = "http://frankfurt.solana.blockrazor.xyz:443/sendTransaction";
let health_endpoint = "http://frankfurt.solana.blockrazor.xyz:443/health";
let mainnetrpc = "";
let authkey = "";
let privatekey ="";
let publickey = ""; // Target public key
```

Run:

```bash
cargo run --bin mode-http-fast
```

---

### HTTP — Sandwich Mitigation

`src/mode_http_sandwichMitigation.rs`

```rust
let http_endpoint = "http://frankfurt.solana.blockrazor.xyz:443/sendTransaction";
let health_endpoint = "http://frankfurt.solana.blockrazor.xyz:443/health";
let mainnetrpc = "";
let authkey = "";
let privatekey = "";
let publickey = ""; 
let amount: u64 = 200_000;
let tipamount: u64 = 1_000_000;
let safe_window: u32 = 5;
let revert_protection = false;
let mode = "sandwichMitigation";
```

Run:

```bash
cargo run --bin mode-http-sandwichMitigation
```

---

## 🛡️ Security Notes

* Always keep your **authKey** and **private key** secure.
* Use environment variables or secret managers instead of hardcoding sensitive data.
* Test in devnet before deploying to mainnet.

---

## 📜 License

MIT License © BlockRazor Inc.

👉 Do you want me to also make a **developer-oriented version** of the README (with Cargo features, contribution guide, and tests), or keep it **trader-user focused** like above?
```
