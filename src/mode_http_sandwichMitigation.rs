use base64::{engine::general_purpose, Engine as _};
use bincode;
use rand::Rng;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_system_interface::instruction::transfer;
use std::str::FromStr;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Serialize)]
struct SendRequest {
    transaction: String,
    mode: String,
    #[serde(rename = "safeWindow")]
    safe_window: u32,
    #[serde(rename = "revertProtection")]
    revert_proctection: bool,
}

#[derive(Deserialize, Debug)]
struct SendResponse {
    signature: String,
}

#[derive(Deserialize, Debug)]
struct HealthResponse {
    result: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let tip_accounts = [
        "Gywj98ophM7GmkDdaWs4isqZnDdFCW7B46TXmKfvyqSm",
        "FjmZZrFvhnqqb9ThCuMVnENaM3JGVuGWNyCAxRJcFpg9",
        "6No2i3aawzHsjtThw81iq1EXPJN6rh8eSJCLaYZfKDTG",
        "A9cWowVAiHe9pJfKAj3TJiN9VpbzMUq6E4kEvf5mUT22",
        "68Pwb4jS7eZATjDfhmTXgRJjCiZmw1L7Huy4HNpnxJ3o",
        "4ABhJh5rZPjv63RBJBuyWzBK3g9gWMUQdTZP2kiW31V9",
        "B2M4NG5eyZp5SBQrSdtemzk5TqVuaWGQnowGaCBt8GyM",
        "5jA59cXMKQqZAVdtopv8q3yyw9SYfiE3vUCbt7p8MfVf",
        "5YktoWygr1Bp9wiS1xtMtUki1PeYuuzuCF98tqwYxf61",
        "295Avbam4qGShBYK7E9H5Ldew4B3WyJGmgmXfiWdeeyV",
        "EDi4rSy2LZgKJX74mbLTFk4mxoTgT6F7HxxzG2HBAFyK",
        "BnGKHAC386n4Qmv9xtpBVbRaUTKixjBe3oagkPFKtoy6",
        "Dd7K2Fp7AtoN8xCghKDRmyqr5U169t48Tw5fEd3wT9mq",
        "AP6qExwrbRgBAVaehg4b5xHENX815sMabtBzUzVB4v8S",
    ];
    // Create a shared HTTP client with keep-alive support
    let client = reqwest::Client::new();

    // Perform initial health check to warm up the connection
    ping_health(&client, health_endpoint, authkey).await?;

    // Start background health pinger to keep the connection alive
    let health_endpoint_clone = health_endpoint.to_string();
    let authkey_clone = authkey.to_string();
    let client_clone = client.clone();
    tokio::spawn(async move {
        loop {
            if let Err(e) = ping_health(&client_clone, &health_endpoint_clone, &authkey_clone).await
            {
                eprintln!("Health check failed: {:?}", e);
            }
            sleep(Duration::from_secs(30)).await;
        }
    });

    // Send Solana transaction
    send_transaction(
        &client,
        mainnetrpc,
        authkey,
        privatekey,
        publickey,
        &tip_accounts,
        tipamount,
        amount,
        mode,
        safe_window,
        revert_protection,
        http_endpoint,
    )
    .await?;

    Ok(())
}

/// Perform GET /health to keep HTTP connection alive
async fn ping_health(
    client: &reqwest::Client,
    health_endpoint: &str,
    authkey: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert("apikey", HeaderValue::from_str(authkey)?);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let res = client.get(health_endpoint).headers(headers).send().await?;

    let body = res.text().await?;
    let parsed: Result<HealthResponse, _> = serde_json::from_str(&body);
    if let Ok(hr) = parsed {
        println!("Health result: {}", hr.result);
    } else {
        return Err("Failed to parse health response".into());
    }
    Ok(())
}

/// Build and send a base64-encoded transaction via HTTP POST
async fn send_transaction(
    client: &reqwest::Client,
    mainnetrpc: &str,
    authkey: &str,
    privatekey: &str,
    publickey: &str,
    tip_accounts: &[&str],
    tipamount: u64,
    amount: u64,
    mode: &str,
    safe_window: u32,
    revert_protection: bool,
    http_endpoint: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let from = Keypair::from_base58_string(privatekey);
    let receiver = Pubkey::from_str(publickey)?;
    let tip = Pubkey::from_str(tip_accounts[rand::thread_rng().gen_range(0..tip_accounts.len())])?;

    let rpcurl = String::from(mainnetrpc);
    let connection = RpcClient::new_with_commitment(rpcurl, CommitmentConfig::confirmed());
    let recent_blockhash = connection
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash.");

    let ix_tip = transfer(&from.pubkey(), &tip, tipamount);
    let ix_main = transfer(&from.pubkey(), &receiver, amount);

    let tx = Transaction::new_signed_with_payer(
        &[ix_tip, ix_main],
        Some(&from.pubkey()),
        &[&from],
        recent_blockhash,
    );

    let serialized = bincode::serialize(&tx)?;
    let base64_encoded = general_purpose::STANDARD.encode(serialized);
    let request = SendRequest {
        transaction: base64_encoded,
        mode: mode.to_string(),
        safe_window: safe_window,
        revert_proctection: revert_protection,
    };
    let mut headers = HeaderMap::new();
    headers.insert("apikey", HeaderValue::from_str(authkey)?);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let res = client
        .post(http_endpoint)
        .headers(headers)
        .json(&request)
        .send()
        .await?;

    let text = res.text().await?;
    let parsed: Result<SendResponse, _> = serde_json::from_str(&text);
    match parsed {
        Ok(r) => {
            println!("Transaction Signature: {}", r.signature); // use field
        }
        Err(_) => println!("RAW RESPONSE: {}", text),
    }

    Ok(())
}
