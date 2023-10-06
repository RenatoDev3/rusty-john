
use ethers::prelude::*;
use rusty_john::utils::*;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv::dotenv().ok();
    let config = Config::new().await;
    println!("[STARTING]");

    let stream = config.wss.subscribe_pending_txs().await?;
    let mut tx_stream = stream.transactions_unordered(usize::MAX);
    while let Some(tx) = tx_stream.next().await {
        if let Ok(tx) = tx {
            println!("[TX] {:?}", tx.hash);
        }
    }
    
    Ok(())
}
