use async_trait::async_trait;
use dotenv::dotenv;
use celestia_rpc::{ BlobClient, Client };
use celestia_types::{ Blob, TxConfig, nmt::Namespace };
use crate::adapters::types::DALayerFunctions;

pub struct CelestiaAdapter;

#[async_trait]
impl DALayerFunctions for CelestiaAdapter {
    async fn push(&self, namespace_id: &[u8], data: &str) -> Option<u64> {
        dotenv().ok();
        let url = std::env::var("RPC_NODE_CELESTIA").expect("RPC_NODE_CELESTIA not set");
        let token = std::env::var("AUTH_TOKEN_CELESTIA").expect("TOKEN not set");

        let client = Client::new(&url, Some(&token)).await.expect("Failed Creating Client");
        let namespace = Namespace::new_v0(&namespace_id).expect("Invalid namespace");
        let blob = Blob::new(namespace, b"kulkarni-shri-store-this".to_vec()).expect(
            "Blob creating failed"
        );
        let height = client.blob_submit(&[blob.clone()], TxConfig::default()).await.ok()?;
        println!("Pushed data to Celestia {}", height);

        Some(height)
    }

    async fn pull(&self) -> String {
        "Pulling data from Celestia".to_string()
    }

    async fn get_block(&self) -> String {
        "Getting block from Celestia".to_string()
    }
}
