use std::cmp::max;
use subxt::{
    tx::PairSigner,
    ext::{
        sp_core::{sr25519, Pair},
        sp_runtime::AccountId32,
    },
    OnlineClient,
    PolkadotConfig,
};
use subxt::config::polkadot::PolkadotExtrinsicParamsBuilder as Params;
use crate::consts::*;

#[subxt::subxt(runtime_metadata_path = "./data/metadata.scale")]
pub mod polkadot {}

pub async fn nominate_all(api: &OnlineClient<PolkadotConfig>, acc_seed_accounts : &[sr25519::Pair]) -> Result<(), Box<dyn std::error::Error>> {
    // Nominate all to the first active validator.
    let validators_addr = polkadot::storage().session().validators();
    let validators = api.storage().at_latest().await?.fetch(&validators_addr).await?.unwrap();
    if 0<validators.len() {
        // Bond tokens
        for i in 0..NB_TEST_ACCOUNTS {
            let acc_seed_account_id: AccountId32 = acc_seed_accounts[i as usize].public().into();
            let tx = polkadot::tx().staking().bond(
                acc_seed_account_id.clone().into(),
                max(EXISTENTIAL_DEPOSIT, TEST_ACCOUNT_FUNDING / 10 - 10 * EDG - EXISTENTIAL_DEPOSIT * (NB_TEST_ACCOUNTS - 1 - i) as u128),
                polkadot::runtime_types::pallet_staking::RewardDestination::Account(acc_seed_account_id.into()),
            );
            let tx_params = Params::new()
                .tip(0).build();
            let acc_signer = PairSigner::new(acc_seed_accounts[i as usize].clone());
            // submit the transaction:
            let hash = api.tx().sign_and_submit(&tx, &acc_signer, tx_params).await?;
            println!("Bonding extrinsic submitted for test account {}: {}",i, hash);
        }
        // Nominate
        for i in 0..NB_TEST_ACCOUNTS {
            let tx = polkadot::tx().staking().nominate(
                vec![validators[0].clone().into()]
            );
            let tx_params = Params::new()
                .tip(0).build();
            let acc_signer = PairSigner::new(acc_seed_accounts[i as usize].clone());
            // submit the transaction:
            let hash = api.tx().sign_and_submit(&tx, &acc_signer, tx_params).await?;
            println!("Nominating extrinsic submitted for test account {}: {}",i, hash);
        }
    }
    Ok(())
}