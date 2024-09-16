use crate::adapters::{
    celestia::CelestiaAdapter,
    eigen_layer::EigenLayerAdapter,
    types::{ DAConfiguration, DATypes, DataAvailabilityAdapter },
};

pub fn get_da_config() -> DataAvailabilityAdapter {
    let celestia_config = DAConfiguration {
        rpc_node: String::from("http://celestia_rpc_url"),
        token: String::from("some_meta_data"),
    };

    let celestia_adapter = DataAvailabilityAdapter {
        adapter_type: DATypes::Celestia,
        config: celestia_config,
        adapter_impl: std::sync::Arc::new(CelestiaAdapter),
    };

    let eigenlayer_config = DAConfiguration {
        rpc_node: String::from("http://eigenlayer_rpc_url"),
        token: String::from("some_meta_data"),
    };

    let eigenlayer_adapter = DataAvailabilityAdapter {
        adapter_type: DATypes::EigenLayer,
        config: eigenlayer_config,
        adapter_impl: std::sync::Arc::new(EigenLayerAdapter),
    };
    celestia_adapter
}
