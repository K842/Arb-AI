//! ArbAI - AI Risk Scoring Contract (Stylus SDK 0.5.1)

#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use stylus_sdk::{prelude::*, alloy_primitives::U256};

/// Contract Storage
sol_storage! {
    #[entrypoint]
    pub struct ArbAI {
        uint256 query_count;
        mapping(address => uint256) last_score;
    }
}

#[public]
impl ArbAI {
    /// Query risk score using 5 features:
    /// f0, f1, f2, f3, f4 → values (0–100)
    ///
    /// Score = weighted_sum(min=0, max=100)
    pub fn query_risk(&mut self, inputs: Vec<U256>) -> U256 {
        if inputs.len() != 5 {
            return U256::from(0);
        }

        // Extract raw limb values
        let f0 = inputs[0].as_limbs()[0] as u128;
        let f1 = inputs[1].as_limbs()[0] as u128;
        let f2 = inputs[2].as_limbs()[0] as u128;
        let f3 = inputs[3].as_limbs()[0] as u128;
        let f4 = inputs[4].as_limbs()[0] as u128;

        // Weighted model (simple linear ML model)
        let score = (f0 * 20 + f1 * 30 + f2 * 20 + f3 * 15 + f4 * 15) / 100;
        let final_score = core::cmp::min(score, 100);

        // Save for caller
        let caller = msg::sender();
        self.last_score.insert(caller, U256::from(final_score));

        // Increment total queries
        let count = self.query_count.get();
        self.query_count.set(count + U256::from(1));

        U256::from(final_score)
    }

    /// Get stored risk score for an address
    pub fn get_score(&self, who: Address) -> U256 {
        self.last_score.get(who)
    }

    /// Total number of queries executed
    pub fn get_query_count(&self) -> U256 {
        self.query_count.get()
    }

    pub fn name() -> String {
        "ArbAI".into()
    }

    pub fn version() -> String {
        "0.1.0".into()
    }
}
