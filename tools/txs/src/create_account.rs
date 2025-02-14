use anyhow::{ Result};
// use libra_config::extension::faucet_client_ext::FaucetClientExt;
use zapatos_types::account_address::AccountAddress;

pub async fn run(account_address: &str, _coins: u64) -> Result<()> {
    // let faucet_client = FaucetClient::default()?;
    let _account_address = AccountAddress::from_hex_literal(account_address)?;

    // if coins == 0 {
    //     faucet_client
    //         .create_account(account_address)
    //         .await
    //         .context(format!(
    //             "Failed to create account {}",
    //             account_address.to_hex_literal()
    //         ))?;
    // } else {
    //     faucet_client
    //         .fund(account_address, coins)
    //         .await
    //         .context(format!("Failed to create account {}", account_address))?;
    // }

    println!("Success!");
    Ok(())
}
