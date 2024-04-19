#[macro_use]
extern crate log;

use color_eyre::eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "info");
    color_eyre::install()?;
    pretty_env_logger::init();

    let client = secret_rpc::SecretRPC::new()
        .host("http://lcd.testnet.secretsaturn.net")
        .enclave_key("e2b40597d50457d95290bdee480b8bc3400e9f40c2a5d69c9519f1fee2e24933")
        .chain_id("secret-4")
        .connect()?;

    info!(target: "auth", "Testing 'auth' queries");

    let resp = client
        .auth_account(secret_rpc::account::a().addr().to_string())
        .await?;

    // There are 4 different possible types of accounts associated with an address
    use secret_rpc::query::auth::Account;
    match resp {
        Account::BaseAccount(base) => {
            info!(target: "auth", "{:?}", base)
        }
        Account::ModuleAccount(module) => {
            info!(target: "auth", "{:?}", module)
        }
        Account::ContinuousVestingAccount(continuous) => {
            info!(target: "auth", "{:?}", continuous)
        }
        Account::DelayedVestingAccount(delayed) => {
            info!(target: "auth", "{:?}", delayed)
        }
    };

    use cosmrs::proto::cosmos::base::query::v1beta1::PageRequest;
    let resp = client
        .auth_accounts(Some(PageRequest {
            key: vec![],
            offset: 0,
            limit: 1,
            count_total: true,
            reverse: false,
        }))
        .await?;
    info!(target: "auth", "{resp:?}");

    let resp = client.auth_params().await?;
    info!(target: "auth", "{resp:?}");

    let resp = client.auth_module_account_by_name("gov").await?;
    info!(target: "auth", "{resp:?}");

    Ok(())
}
