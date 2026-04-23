/// # Simulation helpers for cross-contract integration tests (SW-FE-001)
///
/// Centralises the repetitive setup patterns found across the integration test
/// suite so each test module stays focused on the scenario under test.
///
/// ## Helpers
///
/// | Helper | Purpose |
/// |--------|---------|
/// | `advance_ledger`   | Advance the mock ledger by `delta` sequences |
/// | `set_ledger_seq`   | Set the mock ledger to an absolute sequence number |
/// | `fund_and_mint`    | Fund the reward contract and mint a voucher in one call |
/// | `assert_balances`  | Assert TYC balances for multiple (address, expected) pairs |
#[cfg(test)]
pub mod helpers {
    use crate::fixture::Fixture;
    use soroban_sdk::{
        testutils::{Ledger, LedgerInfo},
        Address,
    };

    /// Advance the mock ledger sequence by `delta` steps.
    ///
    /// Preserves all other `LedgerInfo` fields from the current state so
    /// tests that only care about sequence numbers don't need to set the
    /// full struct.
    pub fn advance_ledger(f: &Fixture, delta: u32) {
        let current = f.env.ledger().get();
        f.env.ledger().set(LedgerInfo {
            sequence_number: current.sequence_number + delta,
            timestamp: current.timestamp + delta as u64 * 5,
            ..current
        });
    }

    /// Set the mock ledger to an absolute sequence number.
    pub fn set_ledger_seq(f: &Fixture, seq: u32) {
        let current = f.env.ledger().get();
        f.env.ledger().set(LedgerInfo {
            sequence_number: seq,
            timestamp: seq as u64 * 5,
            ..current
        });
    }

    /// Fund the reward contract with `tyc_value` TYC and mint a voucher for
    /// `recipient`. Returns the minted `token_id`.
    ///
    /// This collapses the two-step "mint TYC to contract, then mint voucher"
    /// pattern that appears in every reward-flow test.
    pub fn fund_and_mint(f: &Fixture, recipient: &Address, tyc_value: u128) -> u128 {
        f.mint_tyc(&f.reward_id, tyc_value as i128);
        f.reward.mint_voucher(&f.admin, recipient, &tyc_value)
    }

    /// Assert TYC balances for a slice of `(address, expected_balance)` pairs.
    ///
    /// Produces a clear failure message that includes the index and expected
    /// value, making multi-address assertions easy to diagnose.
    pub fn assert_balances(f: &Fixture, checks: &[(&Address, i128)]) {
        for (i, (addr, expected)) in checks.iter().enumerate() {
            assert_eq!(
                f.tyc_balance(addr),
                *expected,
                "balance mismatch at index {i}: expected {expected}"
            );
        }
    }
}
