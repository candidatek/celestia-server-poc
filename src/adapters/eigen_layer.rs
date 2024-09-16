use crate::adapters::types::DALayerFunctions;
use async_trait::async_trait;

pub struct EigenLayerAdapter;
#[async_trait]
impl DALayerFunctions for EigenLayerAdapter {
    async fn push(&self, namespace_id: &[u8], data: &str) -> Option<u64> {
        println!("Pushing data to EigenLayer {:?} {}", namespace_id, data);
        Some(20)
    }

    async fn pull(&self) -> String {
        "Pulling data from EigenLayer".to_string()
    }

    async fn get_block(&self) -> String {
        "Getting block from EigenLayer".to_string()
    }
}