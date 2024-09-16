use std::sync::Arc;
use async_trait::async_trait;

pub enum DATypes {
    Celestia,
    EigenLayer,
}

pub struct DAConfiguration {
    pub rpc_node: String,
    pub token: String,
}

#[async_trait]
pub trait DALayerFunctions {
    async fn push(&self, namespace_id: &[u8], data: &str) -> Option<u64>;
    async fn pull(&self) -> String;
    async fn get_block(&self) -> String;
}

pub struct DataAvailabilityAdapter {
    pub adapter_type: DATypes,
    pub config: DAConfiguration,
    pub adapter_impl: Arc<dyn DALayerFunctions + Send + Sync>,
}