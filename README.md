# Solana-trader-client-rust
example for solana-trader-client in Rust

# Document
see [document](https://blockrazor.gitbook.io/blockrazor/solana/send-transaction/rust)

# Quickstart

1. **Download git repository**

   `git clone https://github.com/BlockRazorinc/solana-trader-client-rust.git`

2. **Change directory**

   `cd solana-trader-client-rust`

3. **Download dependencies**

   install [protoc](https://protobuf.dev/installation/#package-manager), then `cargo build`

4. **Edit src/client.rs**

	```
	// BlockRazor relay endpoint address
    let blzendpoint = "http://frankfurt.solana-grpc.blockrazor.xyz:80";
    // replace your solana rpc endpoint
	let mainnetrpc = "";
    // replace your authKey
    let authkey = "";
    // relace your private key(base58)
	let privatekey = "";
	// send mode
    let mode = "fast";

    // tip amount
	let tipamount  = 1_000_000;
	```

5. **Run mode-fast example**
   
   `cargo run --bin mode-fast`

6. **Run mode-sandwichMitigation example**
   
   `cargo run --bin mode-sandwichMitigation`
