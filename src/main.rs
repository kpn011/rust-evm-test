use ethers::{core::types::TransactionRequest, middleware::SignerMiddleware, providers::{Http, Provider}, signers::{LocalWallet, Signer}, utils::parse_ether};
use dotenv::dotenv;
use rand::Rng;
use std::env;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let rpc_url = env::var("SEPOLIA_RPC_URL").expect("Missing RPC URL");
    let private_key = env::var("PRIVATE_KEY").expect("Missing private key");
    let sender_address = env::var("SENDER_ADDRESS").expect("Missing sender address");
    
    let provider = Provider::<Http>::try_from(rpc_url)?.interval(Duration::from_millis(500));
    let chain_id = provider.get_chainid().await?.as_u64();
    
    let wallet: LocalWallet = private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);
    
    let recipients = vec![
        "0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B",
        "0x1Db3439a222C519ab44bb1144fC28167b4Fa6EE6",
        "0x3fC91A3afd70395Cd496C647d5a6CC9D4B2b7FAD",
    ];
    
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..recipients.len());
    let recipient = recipients[random_index].parse()?;
    
    let min_amount = parse_ether("0.0001")?;
    let max_amount = parse_ether("0.001")?;
    let random_amount = rng.gen_range(min_amount..max_amount);
    
    let balance = client.get_balance(sender_address.parse()?, None).await?;
    
    if balance < random_amount {
        return Err("Insufficient balance".into());
    }
    
    let gas_price = client.get_gas_price().await?;
    let nonce = client.get_transaction_count(sender_address.parse()?, None).await?;
    
    let tx = TransactionRequest::new()
        .to(recipient)
        .value(random_amount)
        .gas(21000)
        .gas_price(gas_price)
        .nonce(nonce)
        .chain_id(chain_id);
    
    let pending_tx = client.send_transaction(tx, None).await?;
    let tx_hash = pending_tx.tx_hash();
    
    println!("Transaction sent: 0x{}", hex::encode(tx_hash.as_bytes()));
    
    let receipt = pending_tx.await?;
    
    match receipt {
        Some(receipt) => {
            println!("Confirmed in block: {:?}", receipt.block_number);
        }
        None => println!("Receipt not found"),
    }
    
    Ok(())
}
