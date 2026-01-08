use ethers::{
    core::types::TransactionRequest,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    utils::parse_ether,
};
use dotenv::dotenv;
use rand::Rng;
use std::env;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting EVM Auto-Sender Bot...");
    
    dotenv().ok();
    
    let rpc_url = env::var("SEPOLIA_RPC_URL")
        .expect("❌ SEPOLIA_RPC_URL must be set in .env");
    
    let private_key = env::var("PRIVATE_KEY")
        .expect("❌ PRIVATE_KEY must be set in .env");
    
    let sender_address = env::var("SENDER_ADDRESS")
        .expect("❌ SENDER_ADDRESS must be set in .env");
    
    println!("🔗 Connecting to Sepolia...");
    let provider = Provider::<Http>::try_from(rpc_url)?
        .interval(Duration::from_millis(500));
    
    let chain_id = provider.get_chainid().await?.as_u64();
    println!("✅ Connected to Chain ID: {}", chain_id);
    
    println!("👛 Setting up wallet...");
    let wallet: LocalWallet = private_key
        .parse::<LocalWallet>()?
        .with_chain_id(chain_id);
    
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);
    
    let recipients = vec![
        "0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B", // Testnet address 1
        "0x1Db3439a222C519ab44bb1144fC28167b4Fa6EE6", // Testnet address 2
        "0x3fC91A3afd70395Cd496C647d5a6CC9D4B2b7FAD", // Testnet address 3
        "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045", // Testnet address 4
        "0x71C7656EC7ab88b098defB751B7401B5f6d8976F", // Testnet address 5
    ];
    
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..recipients.len());
    let recipient = recipients[random_index].parse()?;
    
    println!("🎯 Selected recipient: {}", recipient);
    
    let min_amount = parse_ether("0.0001")?;
    let max_amount = parse_ether("0.001")?;
    let random_amount = rng.gen_range(min_amount..max_amount);
    
    let amount_eth = ethers::utils::format_ether(random_amount);
    println!("💰 Amount to send: {} ETH", amount_eth);
    
    println!("📊 Checking balance...");
    let balance = client.get_balance(sender_address.parse()?, None).await?;
    let balance_eth = ethers::utils::format_ether(balance);
    println!("📈 Current balance: {} ETH", balance_eth);
    
    if balance < random_amount {
        return Err("❌ Insufficient balance".into());
    }
    
    println!("🔨 Building transaction...");
    let gas_price = client.get_gas_price().await?;
    let nonce = client
        .get_transaction_count(sender_address.parse()?, None)
        .await?;
    
    let tx = TransactionRequest::new()
        .to(recipient)
        .value(random_amount)
        .gas(21000) 
        .gas_price(gas_price)
        .nonce(nonce)
        .chain_id(chain_id);
    
    println!("📤 Sending transaction...");
    let pending_tx = client.send_transaction(tx, None).await?;
    let tx_hash = pending_tx.tx_hash();
    
    println!("✅ Transaction sent!");
    println!("🔗 Hash: 0x{}", hex::encode(tx_hash.as_bytes()));
    println!("🌐 Explorer: https://sepolia.etherscan.io/tx/0x{}", hex::encode(tx_hash.as_bytes()));
    
    println!("⏳ Waiting for confirmation...");
    let receipt = pending_tx.await?;
    
    match receipt {
        Some(receipt) => {
            println!("🎉 Transaction confirmed!");
            println!("📦 Block: {:?}", receipt.block_number);
            println!("⛽ Gas used: {:?}", receipt.gas_used);
            println!("📊 Status: {}", if receipt.status == Some(1.into()) { "✅ Success" } else { "❌ Failed" });
        }
        None => {
            println!("⚠️ Transaction receipt not found (might still be pending)");
        }
    }
    
    println!("✨ Bot execution completed!");
    Ok(())
}
