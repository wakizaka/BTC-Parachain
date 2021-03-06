use super::*;
use crate::Module as StakedRelayers;
use bitcoin::formatter::Formattable;
use bitcoin::types::{
    Address, BlockBuilder, H256Le, RawBlockHeader, TransactionBuilder, TransactionInputBuilder,
    TransactionOutput,
};
use btc_relay::Module as BtcRelay;
use collateral::Module as Collateral;
use exchange_rate_oracle::Module as ExchangeRateOracle;
use frame_benchmarking::{account, benchmarks};
use frame_system::RawOrigin;
// use pallet_timestamp::Now;
use sp_core::U256;
use sp_std::prelude::*;
use vault_registry::types::{Vault, Wallet};
use vault_registry::Module as VaultRegistry;

benchmarks! {
    _ {}

    register_staked_relayer {
        let origin: T::AccountId = account("Origin", 0, 0);
        let u in 100 .. 1000;
    }: _(RawOrigin::Signed(origin.clone()), u.into())
    verify {
        assert_eq!(<InactiveStakedRelayers<T>>::get(origin).stake, u.into());
    }

    deregister_staked_relayer {
        let origin: T::AccountId = account("Origin", 0, 0);
        let stake = 100;
        <ActiveStakedRelayers<T>>::insert(&origin, ActiveStakedRelayer{stake: stake.into()});
        <ActiveStakedRelayersCount>::set(1);
        Collateral::<T>::lock_collateral(&origin, stake.into()).unwrap();
    }: _(RawOrigin::Signed(origin))

    activate_staked_relayer {
        let origin: T::AccountId = account("Origin", 0, 0);
        let stake = 100;
        let height = 0;
        StakedRelayers::<T>::add_inactive_staked_relayer(&origin, stake.into(), StakedRelayerStatus::Bonding(height.into()));
    }: _(RawOrigin::Signed(origin))

    deactivate_staked_relayer {
        let origin: T::AccountId = account("Origin", 0, 0);
        let stake = 100;
        StakedRelayers::<T>::add_active_staked_relayer(&origin, stake.into());
    }: _(RawOrigin::Signed(origin))

    suggest_status_update {
        let origin: T::AccountId = account("Origin", 0, 0);
        let stake = 100;
        let deposit = 1000;
        let status_code = StatusCode::Error;
        StakedRelayers::<T>::add_active_staked_relayer(&origin, stake.into());
    }: _(RawOrigin::Signed(origin), deposit.into(), status_code, None, None, None, vec![])

    vote_on_status_update {
        let origin: T::AccountId = account("Origin", 0, 0);
        let stake = 100;
        StakedRelayers::<T>::add_active_staked_relayer(&origin, stake.into());
        let status_update = StatusUpdate::default();
        let status_update_id = StakedRelayers::<T>::insert_active_status_update(status_update);
    }: _(RawOrigin::Signed(origin), status_update_id, true)

    force_status_update {
        let origin: T::AccountId = account("Origin", 0, 0);
        let status_code = StatusCode::Error;
    }: _(RawOrigin::Signed(origin), status_code, None, None)

    slash_staked_relayer {
        let origin: T::AccountId = account("Origin", 0, 0);
        let staked_relayer: T::AccountId = account("Vault", 0, 0);
        let stake = 100;
        StakedRelayers::<T>::add_active_staked_relayer(&staked_relayer, stake.into());
        Collateral::<T>::lock_collateral(&staked_relayer, stake.into()).unwrap();

    }: _(RawOrigin::Signed(origin), staked_relayer)

    report_vault_theft {
        let origin: T::AccountId = account("Origin", 0, 0);
        let stake = 100;
        StakedRelayers::<T>::add_active_staked_relayer(&origin, stake.into());

        let vault_address = Address::from([
            126, 125, 148, 208, 221, 194, 29, 131, 191, 188, 252, 119, 152, 228, 84, 126, 223, 8,
            50, 170,
        ]);

        let address = Address::from([0; 20]);

        let vault_id: T::AccountId = account("Vault", 0, 0);
        let mut vault = Vault::default();
        vault.id = vault_id.clone();
        vault.wallet = Wallet::new(H160::from_slice(vault_address.as_bytes()));
        VaultRegistry::<T>::_insert_vault(
            &vault_id,
            vault
        );

        VaultRegistry::<T>::_set_liquidation_vault(vault_id.clone());

        let mut height = 0;

        let block = BlockBuilder::new()
            .with_version(2)
            .with_coinbase(&address, 50, 3)
            .with_timestamp(1588813835)
            .mine(U256::from(2).pow(254.into()));

        let block_hash = block.header.hash();
        let block_header = RawBlockHeader::from_bytes(&block.header.format()).unwrap();
        BtcRelay::<T>::_initialize(block_header, height).unwrap();

        height += 1;

        let value = 0;
        let transaction = TransactionBuilder::new()
            .with_version(2)
            .add_input(
                TransactionInputBuilder::new()
                    .with_coinbase(false)
                    .with_sequence(4294967295)
                    .with_previous_index(1)
                    .with_previous_hash(H256Le::from_bytes_le(&[
                        193, 80, 65, 160, 109, 235, 107, 56, 24, 176, 34, 250, 197, 88, 218, 76,
                        226, 9, 127, 8, 96, 200, 246, 66, 16, 91, 186, 217, 210, 155, 224, 42,
                    ]))
                    .with_script(&[
                        73, 48, 70, 2, 33, 0, 207, 210, 162, 211, 50, 178, 154, 220, 225, 25, 197,
                        90, 159, 173, 211, 192, 115, 51, 32, 36, 183, 226, 114, 81, 62, 81, 98, 60,
                        161, 89, 147, 72, 2, 33, 0, 155, 72, 45, 127, 123, 77, 71, 154, 255, 98,
                        189, 205, 174, 165, 70, 103, 115, 125, 86, 248, 212, 214, 61, 208, 62, 195,
                        239, 101, 30, 217, 162, 84, 1, 33, 3, 37, 248, 176, 57, 161, 24, 97, 101,
                        156, 155, 240, 63, 67, 252, 78, 160, 85, 243, 167, 28, 214, 12, 123, 31,
                        212, 116, 171, 87, 143, 153, 119, 250,
                    ])
                    .build(),
            )
            .add_output(TransactionOutput::p2pkh(value.into(), &address))
            .build();

        let block = BlockBuilder::new()
            .with_previous_hash(block_hash)
            .with_version(2)
            .with_coinbase(&address, 50, 3)
            .with_timestamp(1588813835)
            .add_transaction(transaction.clone())
            .mine(U256::from(2).pow(254.into()));

        let tx_id = transaction.tx_id();
        let tx_block_height = height;
        let proof = block.merkle_proof(&vec![tx_id]).format();
        let raw_tx = transaction.format_with(true);

        let block_header = RawBlockHeader::from_bytes(&block.header.format()).unwrap();
        BtcRelay::<T>::_store_block_header(block_header).unwrap();

    }: _(RawOrigin::Signed(origin), vault_id, tx_id, tx_block_height, proof, raw_tx)

    report_vault_under_liquidation_threshold {
        let origin: T::AccountId = account("Origin", 0, 0);
        let stake = 100;
        StakedRelayers::<T>::add_active_staked_relayer(&origin, stake.into());

        let vault_id: T::AccountId = account("Vault", 0, 0);
        let mut vault = Vault::default();
        vault.id = vault_id.clone();
        vault.issued_tokens = 100_000.into();
        vault.wallet = Wallet::new(H160::zero());
        VaultRegistry::<T>::_insert_vault(
            &vault_id,
            vault
        );
        VaultRegistry::<T>::_set_liquidation_vault(vault_id.clone());

        ExchangeRateOracle::<T>::_set_exchange_rate(1).unwrap();

        let threshold: u128 = 200_000;
        VaultRegistry::<T>::_set_liquidation_collateral_threshold(threshold.into());

    }: _(RawOrigin::Signed(origin), vault_id)

    // FIXME: broken on no_std
    // report_oracle_offline {
    //     let origin: T::AccountId = account("Origin", 0, 0);
    //     let stake = 100;
    //     StakedRelayers::<T>::add_active_staked_relayer(&origin, stake.into());
    //     <Now<T>>::set(<Now<T>>::get() + 10000.into());
    // }: _(RawOrigin::Signed(origin))

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::{ExtBuilder, Test};
    use frame_support::assert_ok;

    #[test]
    fn test_benchmarks() {
        ExtBuilder::build_with(|storage| {
            pallet_balances::GenesisConfig::<Test> {
                balances: vec![
                    (account("Origin", 0, 0), 1 << 32),
                    (account("Vault", 0, 0), 1 << 32),
                ],
            }
            .assimilate_storage(storage)
            .unwrap();

            GenesisConfig::<Test> {
                gov_id: account("Origin", 0, 0),
            }
            .assimilate_storage(storage)
            .unwrap();
        })
        .execute_with(|| {
            assert_ok!(test_benchmark_register_staked_relayer::<Test>());
            assert_ok!(test_benchmark_deregister_staked_relayer::<Test>());
            assert_ok!(test_benchmark_activate_staked_relayer::<Test>());
            assert_ok!(test_benchmark_deactivate_staked_relayer::<Test>());
            assert_ok!(test_benchmark_suggest_status_update::<Test>());
            assert_ok!(test_benchmark_vote_on_status_update::<Test>());
            assert_ok!(test_benchmark_force_status_update::<Test>());
            assert_ok!(test_benchmark_slash_staked_relayer::<Test>());
            assert_ok!(test_benchmark_report_vault_theft::<Test>());
            assert_ok!(test_benchmark_report_vault_under_liquidation_threshold::<
                Test,
            >());
            // assert_ok!(test_benchmark_report_oracle_offline::<Test>());
        });
    }
}
