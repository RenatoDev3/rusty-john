use ethers::prelude::{k256::ecdsa::SigningKey, *};
use std::sync::Arc;

pub struct Config {
    #[allow(dead_code)]
    pub http: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    #[allow(dead_code)]
    pub wss: Arc<Provider<Ws>>,
}

impl Config {
    pub async fn new() -> Self {
        dotenv::dotenv().ok();
        let network = std::env::var("NETWORK_RPC").expect("missing NETWORK_RPC");
        let provider: Provider<Http> = Provider::<Http>::try_from(network).unwrap();
        let middleware = Arc::new(setup_signer(provider.clone()).await);

        let ws_network = std::env::var("NETWORK_WSS").expect("missing NETWORK_WSS");
        let ws_provider: Provider<Ws> = Provider::<Ws>::connect(ws_network).await.unwrap();
        Self {
            http: middleware,
            wss: Arc::new(ws_provider),
        }
    }
}

/// Converts &str to Address.
pub fn address(address: &str) -> Address {
    address.parse::<Address>().unwrap()
}

/// Sets up middleware w/ our private key env var.
pub async fn setup_signer(
    provider: Provider<Http>,
) -> SignerMiddleware<Provider<Http>, Wallet<SigningKey>> {
    let chain_id = provider
        .get_chainid()
        .await
        .expect("Failed to get chain id.");

    let priv_key = std::env::var("PRIVATE_KEY").expect("missing PRIVATE_KEY");
    dotenv::dotenv().ok();

    let wallet = priv_key
        .parse::<LocalWallet>()
        .expect("Failed to parse wallet")
        .with_chain_id(chain_id.as_u64());

    SignerMiddleware::new(provider, wallet)
}