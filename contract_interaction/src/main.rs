extern crate web3;

use std::time;
use web3::contract::{Contract, Options};

#[tokio::main]
async fn main() -> web3::contract::Result<()> {
    let transport = web3::transports::Http::new("http://localhost:8545").unwrap();

    let web3 = web3::Web3::new(transport);
    let accounts = web3.eth().accounts().await?;

    let bytecode = include_str!("../contract/build/DocumentRegistry.bin");
    let json = include_bytes!("../contract/build/DocumentRegistry.abi");

    let registry_contract = Contract::deploy(web3.eth(), json)?
        .confirmations(0)
        .poll_interval(time::Duration::from_secs(10))
        .options(Options::with(|opt| opt.gas = Some(3_000_000.into())))
        .execute(bytecode, (), accounts[0])
        .await?;

    println!("Contract address: {:?}", registry_contract.address());

    let document_hash = "QmXoypizjW3WknFiJnKLwHCnL72vedxjQkDDP1mXWo6uco";

    let result = registry_contract.query(
        "isNotarized",
        String::from(document_hash),
        accounts[0],
        Options::default(),
        None,
    );
    let is_notarised: bool = result.await?;
    println!("is_notarised: {}", is_notarised);

    let notarize_options = Options::with(|opt| opt.gas = Some(3_000_000.into()));
    let tx_hash = registry_contract
        .call(
            "notarizeDocument",
            String::from(document_hash),
            accounts[0],
            notarize_options,
        )
        .await?;

    let result = registry_contract.query(
        "isNotarized",
        String::from(document_hash),
        None,
        Options::default(),
        None,
    );
    let is_notarised: bool = result.await?;
    println!("is_notarised: {}", is_notarised);
    println!("tx_hash: {:?}", tx_hash);

    Ok(())
}
