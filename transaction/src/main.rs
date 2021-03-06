extern crate web3;

use web3::types::{TransactionRequest, U256};

#[tokio::main]
async fn main() -> web3::contract::Result<()> {
    let transport = web3::transports::Http::new("http://localhost:8545").unwrap();

    let web3 = web3::Web3::new(transport);

    let accounts = web3.eth().accounts().await?;

    let balance_before = web3.eth().balance(accounts[1], None).await?;

    let tx = TransactionRequest {
        from: accounts[0],
        to: Some(accounts[1]),
        gas: None,
        gas_price: None,
        max_fee_per_gas: None,
        max_priority_fee_per_gas: None,
        transaction_type: None,
        access_list: None,
        value: Some(U256::from(10000)),
        data: None,
        nonce: None, // uses last nonce by default
        condition: None
    };

    let tx_hash = web3.eth().send_transaction(tx).await?;

    let balance_after = web3.eth().balance(accounts[1], None).await?;

    println!("TX Hash: {:?}", tx_hash);
    println!("Balance before: {}", balance_before);
    println!("Balance after: {}", balance_after);

    Ok(())
}
