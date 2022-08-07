use ethereum_types::U64;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::RpcResult;
use jsonrpsee::http_client::HttpClient;
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::rpc_params;

use crate::jsonrpc::types::Block;

pub struct Eth {
    transport: HttpClient,
}

impl Eth {
    pub fn create(target: &str) -> Self {
        let transport = HttpClientBuilder::default().build(target).unwrap();
        Self { transport }
    }

    pub async fn block_number(&self) -> RpcResult<U64> {
        let params = rpc_params!();
        self.transport.request("eth_blockNumber", params).await
    }

    pub async fn get_block_by_hash(&self) -> RpcResult<Option<Block>> {
        let params = rpc_params!();
        self.transport.request("eth_blockNumber", params).await
    }
}
