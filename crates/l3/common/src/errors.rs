// use starknet_core::types::ContractErrorData;
use starknet_providers::ProviderError;
use starknet_core::types::StarknetError::ContractError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error{
    #[error(transparent)]
    ProviderError(#[from] ProviderError),
}