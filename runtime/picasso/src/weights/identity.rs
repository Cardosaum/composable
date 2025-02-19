
//! Autogenerated weights for `identity`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-02-26, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("picasso-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/composable
// benchmark
// --chain=picasso-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=20
// --raw
// --output=runtime/picasso/src/weights
// --log
// error

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `identity`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> identity::WeightInfo for WeightInfo<T> {
	// Storage: Identity Registrars (r:1 w:1)
	fn add_registrar(r: u32, ) -> Weight {
		(26_344_000 as Weight)
			// Standard Error: 28_000
			.saturating_add((645_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	fn set_identity(r: u32, x: u32, ) -> Weight {
		(56_166_000 as Weight)
			// Standard Error: 127_000
			.saturating_add((661_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 21_000
			.saturating_add((992_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:1 w:1)
	fn set_subs_new(s: u32, ) -> Weight {
		(49_754_000 as Weight)
			// Standard Error: 16_000
			.saturating_add((6_933_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:1)
	fn set_subs_old(p: u32, ) -> Weight {
		(49_325_000 as Weight)
			// Standard Error: 10_000
			.saturating_add((2_167_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	}
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity IdentityOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:32)
	fn clear_identity(r: u32, s: u32, x: u32, ) -> Weight {
		(62_247_000 as Weight)
			// Standard Error: 57_000
			.saturating_add((220_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 8_000
			.saturating_add((2_166_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 8_000
			.saturating_add((512_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Identity Registrars (r:1 w:0)
	// Storage: Identity IdentityOf (r:1 w:1)
	fn request_judgement(r: u32, x: u32, ) -> Weight {
		(64_803_000 as Weight)
			// Standard Error: 63_000
			.saturating_add((497_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 10_000
			.saturating_add((918_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	fn cancel_request(r: u32, x: u32, ) -> Weight {
		(58_812_000 as Weight)
			// Standard Error: 48_000
			.saturating_add((259_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 8_000
			.saturating_add((974_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity Registrars (r:1 w:1)
	fn set_fee(r: u32, ) -> Weight {
		(10_307_000 as Weight)
			// Standard Error: 11_000
			.saturating_add((486_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity Registrars (r:1 w:1)
	fn set_account_id(r: u32, ) -> Weight {
		(10_445_000 as Weight)
			// Standard Error: 10_000
			.saturating_add((470_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity Registrars (r:1 w:1)
	fn set_fields(r: u32, ) -> Weight {
		(10_483_000 as Weight)
			// Standard Error: 15_000
			.saturating_add((465_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity Registrars (r:1 w:0)
	// Storage: Identity IdentityOf (r:1 w:1)
	fn provide_judgement(r: u32, x: u32, ) -> Weight {
		(44_087_000 as Weight)
			// Standard Error: 48_000
			.saturating_add((341_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 6_000
			.saturating_add((915_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity IdentityOf (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Identity SuperOf (r:0 w:32)
	fn kill_identity(r: u32, s: u32, x: u32, ) -> Weight {
		(84_830_000 as Weight)
			// Standard Error: 67_000
			.saturating_add((578_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 10_000
			.saturating_add((2_198_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 10_000
			.saturating_add((38_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	fn add_sub(s: u32, ) -> Weight {
		(65_269_000 as Weight)
			// Standard Error: 9_000
			.saturating_add((413_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	fn rename_sub(s: u32, ) -> Weight {
		(20_494_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((147_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	fn remove_sub(s: u32, ) -> Weight {
		(66_920_000 as Weight)
			// Standard Error: 12_000
			.saturating_add((410_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	fn quit_sub(s: u32, ) -> Weight {
		(42_880_000 as Weight)
			// Standard Error: 6_000
			.saturating_add((370_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
