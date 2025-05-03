use ethers::providers::{Middleware, Provider, Ws};
use std::sync::Arc;
use tokio_stream::StreamExt;
use std::env;
use dotenv::dotenv; 


async fn get_transactions(){
    dotenv().ok();
    let ws_url = env::var("WEBSOCKET_URL").expect("WEBSOCKET_URL must be set");
    let ws = Ws::connect(ws_url).await.unwrap();
    let provider = Arc::new(Provider::new(ws));

    let mut pending_txs = provider.subscribe_full_pending_txs().await.unwrap();
    while let Some(tx) = pending_txs.next().await{
        println!();
        println!("Transaction: {:?}", tx);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    get_transactions().await;

    Ok(())
}
