use alloy_network::TransactionBuilder;
use alloy_node_bindings::Anvil;
use alloy_primitives::U256;
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types::TransactionRequest;

#[tokio::main]
async fn main() {
    let node = Anvil::new().try_spawn().unwrap();

    let provider = ProviderBuilder::new()
        .on_builtin(&node.endpoint())
        .await
        .unwrap();

    let alice = provider.get_accounts().await.unwrap()[0];

    let tx: TransactionRequest = TransactionRequest::default()
        .with_to(alice)
        .with_nonce(0)
        .with_chain_id(31337)
        .with_value(U256::from(100))
        .with_gas_limit(21_000)
        .with_max_priority_fee_per_gas(1_000_000_000)
        .with_max_fee_per_gas(20_000_000_000);

    let pending_tx = provider.send_transaction(tx).await.unwrap();

    println!("Pending transaction... {}", pending_tx.tx_hash());

    let txn = provider
        .get_transaction_by_hash(*pending_tx.tx_hash())
        .await
        .unwrap();

    let serialized = serde_json::to_string(&txn).unwrap();
    println!("Serialized: {}", serialized);
}
