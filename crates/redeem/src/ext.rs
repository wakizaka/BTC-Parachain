#[cfg(test)]
use mocktopus::macros::mockable;

#[cfg_attr(test, mockable)]
pub(crate) mod btc_relay {
    use bitcoin::types::H256Le;
    use frame_support::dispatch::DispatchResult;
    use sp_std::vec::Vec;

    pub fn verify_transaction_inclusion<T: btc_relay::Trait>(
        tx_id: H256Le,
        merkle_proof: Vec<u8>,
    ) -> DispatchResult {
        <btc_relay::Module<T>>::_verify_transaction_inclusion(tx_id, merkle_proof, 0, false)
    }

    pub fn validate_transaction<T: btc_relay::Trait>(
        raw_tx: Vec<u8>,
        amount: i64,
        btc_address: Vec<u8>,
        issue_id: Vec<u8>,
    ) -> DispatchResult {
        <btc_relay::Module<T>>::_validate_transaction(raw_tx, amount, btc_address, issue_id)
    }
}

#[cfg_attr(test, mockable)]
pub(crate) mod vault_registry {
    use crate::types::{PolkaBTC, DOT};
    use frame_support::dispatch::{DispatchError, DispatchResult};

    pub fn get_vault_from_id<T: vault_registry::Trait>(
        vault_id: &T::AccountId,
    ) -> Result<
        vault_registry::types::Vault<T::AccountId, T::BlockNumber, PolkaBTC<T>>,
        DispatchError,
    > {
        <vault_registry::Module<T>>::_get_vault_from_id(vault_id)
    }

    pub fn increase_to_be_redeemed_tokens<T: vault_registry::Trait>(
        vault_id: &T::AccountId,
        amount: PolkaBTC<T>,
    ) -> DispatchResult {
        <vault_registry::Module<T>>::_increase_to_be_redeemed_tokens(vault_id, amount)
    }

    pub fn redeem_tokens<T: vault_registry::Trait>(
        vault_id: &T::AccountId,
        tokens: PolkaBTC<T>,
    ) -> DispatchResult {
        <vault_registry::Module<T>>::_redeem_tokens(vault_id, tokens)
    }

    pub fn redeem_tokens_premium<T: vault_registry::Trait>(
        vault_id: &T::AccountId,
        tokens: PolkaBTC<T>,
        premium: DOT<T>,
        redeemer_id: &T::AccountId,
    ) -> DispatchResult {
        <vault_registry::Module<T>>::_redeem_tokens_premium(vault_id, tokens, premium, redeemer_id)
    }

    pub fn redeem_tokens_liquidation<T: vault_registry::Trait>(
        redeemer: &T::AccountId,
        redeem_dot_in_btc: PolkaBTC<T>,
    ) -> DispatchResult {
        <vault_registry::Module<T>>::_redeem_tokens_liquidation(redeemer, redeem_dot_in_btc)
    }

    pub fn decrease_tokens<T: vault_registry::Trait>(
        vault_id: &T::AccountId,
        user_id: &T::AccountId,
        tokens: PolkaBTC<T>,
    ) -> DispatchResult {
        <vault_registry::Module<T>>::_decrease_tokens(vault_id, user_id, tokens)
    }

    pub fn ban_vault<T: vault_registry::Trait>(
        vault_id: T::AccountId,
        height: T::BlockNumber,
    ) -> DispatchResult {
        <vault_registry::Module<T>>::_ban_vault(vault_id, height)
    }

    pub fn ensure_not_banned<T: vault_registry::Trait>(
        vault: &T::AccountId,
        height: T::BlockNumber,
    ) -> DispatchResult {
        <vault_registry::Module<T>>::_ensure_not_banned(vault, height)
    }

    pub fn total_liquidation_value<T: vault_registry::Trait>() -> Result<u128, DispatchError> {
        <vault_registry::Module<T>>::_get_total_liquidation_value()
    }

    pub fn punishment_fee<T: vault_registry::Trait>() -> DOT<T> {
        <vault_registry::Module<T>>::punishment_fee()
    }

    pub fn get_redeem_premium_fee<T: vault_registry::Trait>() -> DOT<T> {
        <vault_registry::Module<T>>::redeem_premium_fee()
    }

    pub fn is_vault_below_premium_threshold<T: vault_registry::Trait>(
        vault_id: &T::AccountId,
    ) -> Result<bool, DispatchError> {
        <vault_registry::Module<T>>::_is_vault_below_premium_threshold(&vault_id)
    }
}

#[cfg_attr(test, mockable)]
pub(crate) mod collateral {
    use crate::types::DOT;
    use frame_support::dispatch::DispatchResult;

    pub fn slash_collateral<T: collateral::Trait>(
        sender: &T::AccountId,
        receiver: &T::AccountId,
        amount: DOT<T>,
    ) -> DispatchResult {
        <collateral::Module<T>>::slash_collateral(sender.clone(), receiver.clone(), amount)
    }
}

#[cfg_attr(test, mockable)]
pub(crate) mod treasury {
    use crate::types::PolkaBTC;
    use frame_support::dispatch::DispatchResult;

    pub fn get_balance<T: treasury::Trait>(account: T::AccountId) -> PolkaBTC<T> {
        <treasury::Module<T>>::get_balance_from_account(account)
    }

    pub fn get_total_supply<T: treasury::Trait>() -> PolkaBTC<T> {
        <treasury::Module<T>>::get_total_supply()
    }

    pub fn lock<T: treasury::Trait>(redeemer: T::AccountId, amount: PolkaBTC<T>) -> DispatchResult {
        <treasury::Module<T>>::lock(redeemer, amount)
    }

    pub fn burn<T: treasury::Trait>(redeemer: T::AccountId, amount: PolkaBTC<T>) -> DispatchResult {
        <treasury::Module<T>>::burn(redeemer, amount)
    }
}

#[cfg_attr(test, mockable)]
pub(crate) mod security {
    use frame_support::dispatch::DispatchResult;
    use primitive_types::H256;
    use security::ErrorCode;
    use sp_std::vec::Vec;

    pub fn is_parachain_error_liquidation<T: security::Trait>() -> bool {
        <security::Module<T>>::_is_parachain_error_liquidation()
    }

    pub fn get_secure_id<T: security::Trait>(id: &T::AccountId) -> H256 {
        <security::Module<T>>::_get_secure_id(id)
    }

    pub fn ensure_parachain_status_running<T: security::Trait>() -> DispatchResult {
        <security::Module<T>>::_ensure_parachain_status_running()
    }

    pub fn ensure_parachain_status_has_only_specific_errors<T: security::Trait>(
        error_codes: Vec<ErrorCode>,
    ) -> DispatchResult {
        <security::Module<T>>::_ensure_parachain_status_has_only_specific_errors(error_codes)
    }
}

#[cfg_attr(test, mockable)]
pub(crate) mod oracle {
    use crate::types::{PolkaBTC, DOT};
    use frame_support::dispatch::DispatchError;

    pub fn btc_to_dots<T: exchange_rate_oracle::Trait>(
        amount: PolkaBTC<T>,
    ) -> Result<DOT<T>, DispatchError> {
        <exchange_rate_oracle::Module<T>>::btc_to_dots(amount)
    }
}
