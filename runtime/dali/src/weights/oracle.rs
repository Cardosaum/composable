
//! Autogenerated weights for `oracle`
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

/// Weight functions for `oracle`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> oracle::WeightInfo for WeightInfo<T> {
	// Storage: Oracle AssetsCount (r:1 w:1)
	// Storage: Oracle AssetsInfo (r:1 w:1)
	fn add_asset_and_info() -> Weight {
		(30_481_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Oracle ControllerToSigner (r:1 w:1)
	// Storage: Oracle SignerToController (r:1 w:1)
	// Storage: Oracle OracleStake (r:1 w:1)
	fn set_signer() -> Weight {
		(112_450_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Oracle ControllerToSigner (r:1 w:0)
	// Storage: Oracle OracleStake (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn add_stake() -> Weight {
		(97_448_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Oracle ControllerToSigner (r:1 w:0)
	// Storage: Oracle OracleStake (r:1 w:1)
	// Storage: Oracle DeclaredWithdraws (r:0 w:1)
	fn remove_stake() -> Weight {
		(36_980_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Oracle ControllerToSigner (r:1 w:1)
	// Storage: Oracle DeclaredWithdraws (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: Oracle SignerToController (r:0 w:1)
	fn reclaim_stake() -> Weight {
		(45_879_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Oracle OracleStake (r:1 w:0)
	// Storage: Oracle Prices (r:1 w:0)
	// Storage: Oracle AssetsInfo (r:1 w:0)
	// Storage: Oracle AnswerInTransit (r:1 w:1)
	// Storage: Oracle PrePrices (r:1 w:1)
	fn submit_price(p: u32, ) -> Weight {
		(56_662_000 as Weight)
			// Standard Error: 9_000
			.saturating_add((294_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Oracle PrePrices (r:1 w:1)
	// Storage: Oracle AnswerInTransit (r:1 w:1)
	fn update_pre_prices(p: u32, ) -> Weight {
		(15_305_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((274_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Oracle PriceHistory (r:1 w:1)
	// Storage: Oracle SignerToController (r:1 w:0)
	// Storage: Oracle AnswerInTransit (r:1 w:1)
	// Storage: Oracle Prices (r:0 w:1)
	// Storage: Oracle PrePrices (r:0 w:1)
	fn update_price(p: u32, ) -> Weight {
		(41_760_000 as Weight)
			// Standard Error: 27_000
			.saturating_add((14_289_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
}
