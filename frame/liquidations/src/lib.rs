#![cfg_attr(not(test), warn(clippy::disallowed_method, clippy::indexing_slicing))] // allow in tests
#![warn(clippy::unseparated_literal_suffix, clippy::disallowed_type)]
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(
	bad_style,
	bare_trait_objects,
	const_err,
	improper_ctypes,
	non_shorthand_field_patterns,
	no_mangle_generic_items,
	overflowing_literals,
	path_statements,
	patterns_in_fns_without_body,
	private_in_public,
	unconditional_recursion,
	unused_allocation,
	unused_comparisons,
	unused_parens,
	while_true,
	trivial_casts,
	trivial_numeric_casts,
	unused_extern_crates
)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

	use codec::FullCodec;
	use composable_traits::{
		defi::{DeFiComposableConfig, DeFiEngine, SellEngine},
		lending::Lending,
		liquidation::Liquidation, time::TimeReleaseFunction,
	};
	use frame_support::{
		traits::{IsType, UnixTime, Get, GenesisBuild},
		PalletId, dispatch::Dispatchable, Twox64Concat, pallet_prelude::{OptionQuery, StorageMap},
	};

	use sp_runtime::DispatchError;

	#[pallet::config]

	pub trait Config: frame_system::Config + DeFiComposableConfig {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type UnixTime: UnixTime;

		type DutchAuction: SellEngine<TimeReleaseFunction>; 

		type LiquidationStrategyId: Default + FullCodec + Into<u64>;
		
		type OrderId: Default + FullCodec;

		type PalletId: Get<PalletId>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (crate) fn deposit_event)]
	pub enum Event<T: Config> {
		PositionWasSentToLiquidation {},
	}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		
	}

	// TODO: real flow to implement:
	// ```plantuml
	// `solves question - how pallet can invoke list of other pallets with different configuration types
	// `so yet sharing some liqudation part and tracing liquidation id
	// dutch_auction_strategy -> liquidation : Create new strategy id
	// dutch_auction_strategy -> liquidation : Add Self Dispatchable call (baked with strategyid)
	// liquidation -> liquidation: Add liquidation order 
	// liquidation -> liquidation: Get Dispatchable by Strategyid
	// liquidation --> dutch_auction_strategy: Invoke Dispatchable
	// dutch_auction_strategy -> dutch_auction_strategy: Get liquidation configuration by id previosly baked into call
	// dutch_auction_strategy --> liqudation: Pop next order
	// dutch_auction_strategy -> dutch_auction_strategy: Start liqudaiton
	// ```
	// for now just build in luqidation here
	#[pallet::storage]
	#[pallet::getter(fn strategies)]
	pub type Strategies<T:Config> = StorageMap<_, Twox64Concat, u64, TimeReleaseFunction, OptionQuery>; 

	impl<T: Config> DeFiEngine for Pallet<T> {
		type MayBeAssetId = T::MayBeAssetId;

		type Balance = T::Balance;

		type AccountId = T::AccountId;
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		_phantom: sp_std::marker::PhantomData<T>,
	}

	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { _phantom: <_>::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T:Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self){
			//LiquidationStr
		}
	}

	

	impl<T: Config> Liquidation for Pallet<T> {
		
		type LiquidationStrategyId = T::LiquidationStrategyId;

		type OrderId = T::OrderId;		

		fn liquidate(
				from_to: &Self::AccountId,
				order: composable_traits::defi::Sell<Self::MayBeAssetId, Self::Balance>,		
				configuration : Vec<Self::LiquidationStrategyId>,
			) -> Result<Self::OrderId, DispatchError> {
				
				todo!()
			}
	}
}
