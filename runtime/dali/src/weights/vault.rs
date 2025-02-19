
//! Autogenerated weights for `vault`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-02-27, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dali-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/composable
// benchmark
// --chain=dali-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=20
// --raw
// --output=runtime/dali/src/weights
// --log
// error

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `vault`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> vault::WeightInfo for WeightInfo<T> {
	// Storage: Vault VaultCount (r:1 w:1)
	// Storage: CurrencyFactory AssetIdRanges (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Vault LpTokensToVaults (r:0 w:1)
	// Storage: Vault Vaults (r:0 w:1)
	fn create() -> Weight {
		(139_575_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: Vault Vaults (r:1 w:0)
	// Storage: Tokens Accounts (r:3 w:3)
	// Storage: Tokens TotalIssuance (r:2 w:1)
	// Storage: Vault CapitalStructure (r:2 w:0)
	// Storage: System Account (r:1 w:1)
	fn deposit() -> Weight {
		(147_843_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Vault Vaults (r:1 w:0)
	// Storage: Tokens Accounts (r:3 w:3)
	// Storage: Vault CapitalStructure (r:2 w:0)
	// Storage: Tokens TotalIssuance (r:2 w:1)
	fn withdraw() -> Weight {
		(120_773_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Vault Vaults (r:1 w:1)
	fn emergency_shutdown() -> Weight {
		(29_450_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Vault Vaults (r:1 w:1)
	fn start_() -> Weight {
		(28_716_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Vault Vaults (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn add_surcharge() -> Weight {
		(78_852_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Vault Vaults (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn claim_surcharge() -> Weight {
		(73_549_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Vault Vaults (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: Vault LpTokensToVaults (r:0 w:1)
	fn delete_tombstoned() -> Weight {
		(25_987_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
