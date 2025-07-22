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

4. **Edit src/mode_grpc_fast.rs**

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
    let mode = "";

    // tip amount
	let tipamount  = 1_000_000;
	```

5. **Run mode-grpc-fast example**
   
   `cargo run --bin mode-grpc-fast`

# GRPC

## fast mode

1. **Edit src/mode_grpc_fast.rs**

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
    let mode = "";

    // tip amount
	let tipamount  = 1_000_000;
	```

2. **Run mode-grpc-fast example**
   
   `cargo run --bin mode-grpc-fast`

## sandwichMitigation mode

1. **Edit src/mode_grpc_sandwichMitigation.rs**

	```
	 // BlockRazor relay endpoint address
    let blzendpoint = "http://frankfurt.solana-grpc.blockrazor.xyz:80";
    // replace your solana rpc endpoint
    let mainnetrpc = "";
    // replace your authKey
    let authkey = "";
    // relace your private key
    let privatekey = "";
    // send mode
    let mode = "sandwichMitigation";
    // safe window
    let safe_window = Some(5);
    // revert protection
    let revert_protection = false;
	```

2. **Run mode-grpc-sandwichMitigation example**
   
   `cargo run --bin mode-grpc-sandwichMitigation`



# HTTP

## fast mode

1. **Edit src/mode_http_fast.rs**

	```
	let http_endpoint = "http://frankfurt.solana.blockrazor.xyz:443/sendTransaction";
   let health_endpoint = "http://frankfurt.solana.blockrazor.xyz:443/health";
   let mainnetrpc = "";
   // replace your authKey
   let authkey = "";
   // relace your private key
   let privatekey ="";
   // relace your target public key
   let publickey = "";
	```

2. **Run mode-http-fast example**
   
   `cargo run --bin mode-http-fast`

## sandwichMitigation mode

1. **Edit src/mode_http_sandwichMitigation.rs**

	```
	 // Configuration values
    let http_endpoint = "http://frankfurt.solana.blockrazor.xyz:443/sendTransaction";
    let health_endpoint = "http://frankfurt.solana.blockrazor.xyz:443/health";
    let mainnetrpc = "";
    // replace your authKey
    let authkey = "";
    // relace your private key
    let privatekey = "";
    // relace your target public key
    let publickey = "";
    // transaction amount
    let amount: u64 = 200_000;
    // tip amount
    let tipamount: u64 = 1_000_000;
    // safe window
    let safe_window: u32 = 5;
    // revert protection
    let revert_protection = false;
    // send mode
    let mode = "sandwichMitigation";
	```

2. **Run mode-http-sandwichMitigation example**
   
   `cargo run --bin mode-http-sandwichMitigationn`