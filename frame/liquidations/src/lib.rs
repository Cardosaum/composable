#![cfg_attr(
	not(test),
	deny(
		clippy::disallowed_method,
		clippy::disallowed_type,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![deny(clippy::unseparated_literal_suffix)]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
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

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod mock;

#[cfg(test)]
mod tests;
mod weights;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

	use codec::{Decode, Encode, FullCodec, MaxEncodedLen};
	use composable_traits::{
		defi::{DeFiComposableConfig, DeFiEngine, Sell, SellEngine},
		liquidation::Liquidation,
		math::WrappingNext,
		time::{LinearDecrease, StairstepExponentialDecrease, TimeReleaseFunction},
	};
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::{OptionQuery, StorageMap, StorageValue},
		traits::{Get, IsType, UnixTime},
		PalletId, Parameter, Twox64Concat,
	};

	#[cfg(feature = "std")]
	use frame_support::traits::GenesisBuild;

	use frame_system::pallet_prelude::OriginFor;
	use scale_info::TypeInfo;
	use sp_runtime::{DispatchError, Permill, Perquintill};
	use sp_std::vec::Vec;

	use crate::weights::WeightInfo;

	#[pallet::config]

	pub trait Config: frame_system::Config + DeFiComposableConfig {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type UnixTime: UnixTime;

		type DutchAuction: SellEngine<
			TimeReleaseFunction,
			OrderId = Self::OrderId,
			MayBeAssetId = <Self as DeFiComposableConfig>::MayBeAssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
		>;

		type LiquidationStrategyId: Default
			+ FullCodec
			+ MaxEncodedLen
			+ WrappingNext
			+ Parameter
			+ Copy;

		type OrderId: Default + FullCodec + MaxEncodedLen + sp_std::fmt::Debug;

		type PalletId: Get<PalletId>;

		// /// when called, engine pops latest order to liquidate and pushes back result
		// type Liquidate: Parameter + Dispatchable<Origin = Self::Origin> + From<Call<Self>>;
		type WeightInfo: WeightInfo;
		type ParachainId: FullCodec + MaxEncodedLen + Default + Parameter + Clone;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (crate) fn deposit_event)]
	pub enum Event<T: Config> {
		PositionWasSentToLiquidation {},
	}

	#[pallet::error]
	pub enum Error<T> {
		NoLiquidationEngineFound,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::add_liquidation_strategy())]
		pub fn add_liquidation_strategy(
			_origin: OriginFor<T>,
			_configuraiton: LiquidationStrategyConfiguration<T::ParachainId>,
		) -> DispatchResultWithPostInfo {
			Err(DispatchError::Other("add_liquidation_strategy: no implemented").into())
		}
	}

	#[pallet::storage]
	#[pallet::getter(fn strategies)]
	pub type Strategies<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::LiquidationStrategyId,
		LiquidationStrategyConfiguration<T::ParachainId>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn strategy_index)]
	#[allow(clippy::disallowed_type)]
	pub type StrategyIndex<T: Config> =
		StorageValue<_, T::LiquidationStrategyId, frame_support::pallet_prelude::ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn default_strategy_index)]
	#[allow(clippy::disallowed_type)]
	pub type DefaultStrategyIndex<T: Config> =
		StorageValue<_, T::LiquidationStrategyId, frame_support::pallet_prelude::ValueQuery>;

	impl<T: Config> DeFiEngine for Pallet<T> {
		type MayBeAssetId = T::MayBeAssetId;

		type Balance = T::Balance;

		type AccountId = T::AccountId;
	}

	#[cfg(feature = "std")]
	#[derive(Default)]
	#[pallet::genesis_config]
	pub struct GenesisConfig;

	impl<T: Config> Pallet<T> {
		pub fn create_strategy_id() -> T::LiquidationStrategyId {
			StrategyIndex::<T>::mutate(|x| {
				*x = x.next();
				*x
			})
		}
	}

	#[derive(Clone, Debug, PartialEq, Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub enum LiquidationStrategyConfiguration<ParachainId> {
		DutchAuction(TimeReleaseFunction),
		UniswapV2 { slippage: Perquintill },
		XcmDex { parachain_id: ParachainId },
		/* Building fully decoupled flow is described bellow. Will avoid that for now.
		 * ```plantuml
		 * `solves question - how pallet can invoke list of other pallets with different configuration types
		 * `so yet sharing some liquidation part and tracing liquidation id
		 * dutch_auction_strategy -> liquidation : Create new strategy id
		 * dutch_auction_strategy -> liquidation : Add Self Dispatchable call (baked with strategyid)
		 * liquidation -> liquidation: Add liquidation order
		 * liquidation -> liquidation: Get Dispatchable by Strategyid
		 * liquidation --> dutch_auction_strategy: Invoke Dispatchable
		 * dutch_auction_strategy -> dutch_auction_strategy: Get liquidation configuration by id previosly baked into call
		 * dutch_auction_strategy --> liquidation: Pop next order
		 * dutch_auction_strategy -> dutch_auction_strategy: Start liquidation
		 * ```
		 *Dynamic { liquidate: Dispatch, minimum_price: Balance }, */
	}

	#[cfg(feature = "std")]
	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {
			let index = Pallet::<T>::create_strategy_id();
			DefaultStrategyIndex::<T>::set(index);

			let linear_ten_minutes = LiquidationStrategyConfiguration::DutchAuction(
				TimeReleaseFunction::LinearDecrease(LinearDecrease { total: 10 * 60 }),
			);
			Strategies::<T>::insert(index, linear_ten_minutes);

			let index = Pallet::<T>::create_strategy_id();
			let exponential =
				StairstepExponentialDecrease { step: 10, cut: Permill::from_rational(95_u32, 100) };
			let exponential = LiquidationStrategyConfiguration::DutchAuction(
				TimeReleaseFunction::StairstepExponentialDecrease(exponential),
			);
			Strategies::<T>::insert(index, exponential);
		}
	}

	impl<T: Config> Liquidation for Pallet<T> {
		type LiquidationStrategyId = T::LiquidationStrategyId;
		type OrderId = T::OrderId;

		fn liquidate(
			from_to: &Self::AccountId,
			order: Sell<Self::MayBeAssetId, Self::Balance>,
			configuration: Vec<Self::LiquidationStrategyId>,
		) -> Result<T::OrderId, DispatchError> {
			let mut configuration = configuration;
			if configuration.is_empty() {
				configuration.push(DefaultStrategyIndex::<T>::get())
			};

			for id in configuration {
				let configuration = Strategies::<T>::get(id);
				if let Some(configuration) = configuration {
					let result = match configuration {
						LiquidationStrategyConfiguration::DutchAuction(configuration) =>
							T::DutchAuction::ask(from_to, order.clone(), configuration),
						_ =>
							return Err(DispatchError::Other(
								"as for now, only auction liquidators implemented",
							)),
					};

					if result.is_ok() {
						Self::deposit_event(Event::<T>::PositionWasSentToLiquidation {});
						return result
					}
				}
			}

			Err(Error::<T>::NoLiquidationEngineFound.into())
		}
	}
}
