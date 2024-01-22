use alloy_primitives::{bytes::Bytes, Address, B256, U256};
use jsonrpsee::proc_macros::rpc;

use crate::BlockId;

#[rpc(client)]
pub trait StupidEth {
    #[method(name = "eth_blockNumber")]
    async fn block_number(&self, parameters: Vec<()>) -> RpcResult<U256>;
    #[method(name = "eth_getStorageAt")]
    async fn get_storage_latest(
        &self,
        address: Address,
        position: U256,
        block: BlockId,
    ) -> RpcResult<B256>;
    #[method(name = "eth_getCode")]
    async fn get_code(&self, address: Address, block: BlockId) -> RpcResult<Bytes>;
}
#[cfg(test)]
mod tests {
    use crate::{client::StupidEthClient, LATEST};
    use alloy_primitives::{Address, U256};
    use jsonrpsee::http_client::HttpClientBuilder;
    use tracing::{debug, error, Level};

    fn setup() -> &'static str {
        let _ = tracing_subscriber::fmt().with_max_level(Level::INFO).try_init();
        let url = "https://eth.llamarpc.com";
        url
    }

    #[tokio::test]
    async fn test_get_block_number() {
        let url = setup();

        let client = HttpClientBuilder::default().build(url).unwrap();

        let block_number = client.block_number(vec![]).await;
        if let Err(e) = &block_number {
            error!("Error fetching block number: {:?}", e);
        }

        assert!(block_number.is_ok(), "Test failed with error: {:?}", block_number.err().unwrap());

        if let Ok(number) = block_number {
            debug!("block number: {:?}", number.to_string());
        }
    }
    #[tokio::test]
    async fn test_get_storage_latest() {
        let url = setup();

        let client = HttpClientBuilder::default().build(url).unwrap();

        let address = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse::<Address>().unwrap();

        let storage = client.get_storage_latest(address, U256::from(0), LATEST).await;
        if let Err(e) = &storage {
            error!("Error fetching storage: {:?}", e);
        }

        assert!(storage.is_ok(), "Test failed with error: {:?}", storage.err().unwrap());

        if let Ok(storage_slot) = storage {
            debug!("storage slot: {:?}", storage_slot);
        }
    }
    #[tokio::test]
    async fn test_get_code() {
        let url = setup();
        let client = HttpClientBuilder::default().build(url).unwrap();
        let address = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse::<Address>().unwrap();

        let code = client.get_code(address, LATEST).await;
        if let Err(e) = &code {
            error!("Error fetching code: {:?}", e);
        }

        assert!(code.is_ok(), "Test failed with error: {:?}", code.err().unwrap());

        if let Ok(code_data) = code {
            debug!("code data: {:#?}", code_data);
        }
    }
}
