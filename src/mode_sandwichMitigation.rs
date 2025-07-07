use base64::{engine::general_purpose, Engine as _};
use bincode;
use rand::Rng;
use server::server_client::ServerClient;
use server::{HealthRequest, SendRequest};
use solana_client::rpc_client::RpcClient;
use solana_program::system_instruction;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;
use std::error::Error;
use std::str::FromStr;
use tonic::metadata::AsciiMetadataValue;
use tonic::transport::Channel;

pub mod server {
    tonic::include_proto!("serverpb");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // BlockRazor relay endpoint address
    let blzendpoint = "http://frankfurt.solana-grpc.blockrazor.xyz:80";
    // replace your solana rpc endpoint
    let mainnetrpc = "";
    // replace your authKey
    let authkey = "";
    // relace your private key(base58)
    let privatekey = "";

    // send mode
    let mode = "sandwichMitigation";

    // tip amount
    let tipamount = 1_000_000;

    let tip_accounts = [
        "Gywj98ophM7GmkDdaWs4isqZnDdFCW7B46TXmKfvyqSm",
        "FjmZZrFvhnqqb9ThCuMVnENaM3JGVuGWNyCAxRJcFpg9",
        "6No2i3aawzHsjtThw81iq1EXPJN6rh8eSJCLaYZfKDTG",
        "A9cWowVAiHe9pJfKAj3TJiN9VpbzMUq6E4kEvf5mUT22",
    ];

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..tip_accounts.len());
    let tip_account = tip_accounts[random_index];

    let channel = Channel::from_shared(blzendpoint.to_string())
        .map_err(|e| Box::<dyn Error>::from(format!("Invalid URI: {}", e)))?
        .connect()
        .await
        .map_err(|e| Box::<dyn Error>::from(format!("Connection error: {}", e)))?;

    let apikeyvalue = AsciiMetadataValue::try_from(authkey)
        .map_err(|e| Box::<dyn Error>::from(format!("Invalid API key format: {}", e)))?;

    let mut client = ServerClient::new(channel);

    let mut request = tonic::Request::new(HealthRequest {});
    request.metadata_mut().insert("apikey", apikeyvalue.clone());
    let response = client.get_health(request).await?;
    println!("get health response={:?}", response.into_inner().status);

    let from = Keypair::from_base58_string(privatekey);
    let frompubkey = Signer::pubkey(&from);
    let topubkey = Pubkey::from_str(tip_account).expect("Failed to parse receivers pubkey");

    let rpcurl = String::from(mainnetrpc);
    let connection = RpcClient::new_with_commitment(rpcurl, CommitmentConfig::confirmed());

    let ix = system_instruction::transfer(&frompubkey, &topubkey, tipamount);
    let recent_blockhash = connection
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash.");
    let tx =
        Transaction::new_signed_with_payer(&[ix], Some(&frompubkey), &[&from], recent_blockhash);

    // tx base64
    let serialized = bincode::serialize(&tx)?;
    let base64_encoded = general_purpose::STANDARD.encode(serialized);
    let safe_window = Some(5);
    let mut tx_request: tonic::Request<SendRequest> = tonic::Request::new(SendRequest {
        transaction: base64_encoded,
        mode: mode.to_string(),
        safe_window,
    });
    tx_request
        .metadata_mut()
        .insert("apikey", apikeyvalue.clone());
    let response = client.send_transaction(tx_request).await?;
    println!("SEND TX RESPONSE={:?}", response);

    Ok(())
}
