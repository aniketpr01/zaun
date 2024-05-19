use common::{call_contract, invoke_contract};
use common::LocalWalletSignerMiddleware;
use starknet_accounts::Execution;
use starknet_core::types::FieldElement;
use std::sync::Arc;
use color_eyre::Result;
use common::model::StarkProofWithSerde;

pub struct VerifyContract {
    client: Arc<LocalWalletSignerMiddleware>,
    address: FieldElement,
}

impl VerifyContract {
    pub fn new(address: FieldElement, client: Arc<LocalWalletSignerMiddleware>) -> Self {
        Self { client, address }
    }

    pub fn verify_and_register_fact(
        &self,
        stark_proof: StarkProofWithSerde,
    ) -> Result<Execution<LocalWalletSignerMiddleware>> {
        let mut calldata = Vec::new();
        calldata.extend(stark_proof.serialize());
        invoke_contract(&self.client, self.address, "verify_and_register_fact", calldata).await
    }

    pub fn verify_and_register_fact_from_contract(
        &self,
        contract_address: ContractAddress,
    ) -> Result<Execution<LocalWalletSignerMiddleware>> {
        let mut calldata = Vec::new();
        calldata.push(contract_address);
        invoke_contract(&self.client, self.address, "verify_and_register_fact_from_contract", calldata).await
    }

    pub async fn is_valid(&self, fact: FieldElement) -> Result<bool> {
        let values = call_contract(&self.client, self.address, "is_valid", vec![fact]).await?;
        if let Some(value) = values.first() {
            Ok(value.to_string() != String::from("0"))
        } else {
            Err(eyre!("Contract error: expected at least one return value"))
        }
    }
}
