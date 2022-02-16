#![cfg_attr(
	not(test),
	warn(
		clippy::disallowed_method,
		clippy::disallowed_type,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)]
#![warn(clippy::unseparated_literal_suffix)]
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

pub mod weights;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use crate::weights::WeightInfo;
	use codec::{Codec, FullCodec};
	use composable_support::validation::{Validate, Validated};
	use composable_traits::{
		currency::LocalAssets, defi::CurrencyPair, dex::CurveAmm, math::SafeArithmetic,
	};
	use core::fmt::Debug;
	use frame_support::{
		pallet_prelude::*,
		traits::fungibles::{Inspect, Transfer},
		transactional, PalletId, RuntimeDebug,
	};
	use frame_system::{
		ensure_signed,
		pallet_prelude::{BlockNumberFor, OriginFor},
	};
	use rust_decimal::prelude::*;
	use sp_arithmetic::traits::Saturating;
	use sp_runtime::{
		traits::{
			AccountIdConversion, BlockNumberProvider, CheckedMul, CheckedSub, Convert, One, Zero,
		},
		ArithmeticError, Permill,
	};

	#[derive(Copy, Clone, PartialEq, Eq)]
	pub enum SaleState {
		NotStarted,
		Ongoing,
		Ended,
	}

	#[derive(RuntimeDebug, Encode, Decode, MaxEncodedLen, Copy, Clone, PartialEq, Eq, TypeInfo)]
	pub struct Sale<BlockNumber> {
		/// Block at which the sale start.
		pub start: BlockNumber,
		/// Block at which the sale stop.
		pub end: BlockNumber,
		/// Initial weight of the base asset of the current pair.
		pub initial_weight: Permill,
		/// Final weight of the base asset of the current pair.
		pub final_weight: Permill,
	}

	impl<BlockNumber: TryInto<u64> + Ord + Copy + Saturating + SafeArithmetic> Sale<BlockNumber> {
		pub(crate) fn current_weights(
			&self,
			current_block: BlockNumber,
		) -> Result<(Permill, Permill), DispatchError> {
			/* NOTE(hussein-aitlahcen): currently only linear

			   Assuming final_weight < initial_weight
			   current_weight = initial_weight - (current - start) / (end - start) * (initial_weight - final_weight)
							  = initial_weight - normalized_current / sale_duration * weight_range
							  = initial_weight - point_in_sale * weight_range
			*/
			let normalized_current_block = current_block.safe_sub(&self.start)?;
			let point_in_sale = Permill::from_rational(
				normalized_current_block.try_into().map_err(|_| ArithmeticError::Overflow)?,
				self.duration().try_into().map_err(|_| ArithmeticError::Overflow)?,
			);
			let weight_range = self
				.initial_weight
				.checked_sub(&self.final_weight)
				.ok_or(ArithmeticError::Underflow)?;
			let current_base_weight = self
				.initial_weight
				.checked_sub(
					&point_in_sale.checked_mul(&weight_range).ok_or(ArithmeticError::Overflow)?,
				)
				.ok_or(ArithmeticError::Underflow)?;
			let current_quote_weight = Permill::one()
				.checked_sub(&current_base_weight)
				.ok_or(ArithmeticError::Underflow)?;
			Ok((current_base_weight, current_quote_weight))
		}
	}

	impl<BlockNumber: Copy + Saturating> Sale<BlockNumber> {
		pub(crate) fn duration(&self) -> BlockNumber {
			// NOTE(hussein-aitlahcen): end > start as previously checked by PoolIsValid.
			self.end.saturating_sub(self.start)
		}
	}

	impl<BlockNumber: Ord> Sale<BlockNumber> {
		pub(crate) fn state(&self, current_block: BlockNumber) -> SaleState {
			if current_block < self.start {
				SaleState::NotStarted
			} else if current_block >= self.end {
				SaleState::Ended
			} else {
				SaleState::Ongoing
			}
		}
	}

	#[derive(RuntimeDebug, Encode, Decode, MaxEncodedLen, Copy, Clone, PartialEq, Eq, TypeInfo)]
	pub struct Pool<AccountId, BlockNumber, AssetId> {
		/// Owner of the pool
		pub owner: AccountId,
		/// Asset pair of the pool along their weight.
		/// Base asset is the project token.
		/// Quote asset is the collateral token.
		pub pair: CurrencyPair<AssetId>,
		/// Sale period of the LBP.
		pub sale: Sale<BlockNumber>,
	}

	#[derive(Copy, Clone, Encode, Decode, MaxEncodedLen, PartialEq, Eq, TypeInfo)]
	pub struct PoolIsValid<T>(PhantomData<T>);

	impl<T: Config> Validate<PoolOf<T>, PoolIsValid<T>> for PoolIsValid<T> {
		fn validate(input: PoolOf<T>) -> Result<PoolOf<T>, &'static str> {
			if input.pair.base == input.pair.quote {
				return Err("Pair elements must be distinct.");
			}

			if input.sale.end <= input.sale.start {
				return Err("Sale end must be after start.");
			}

			if input.sale.duration() > T::MaxSaleDuration::get() {
				return Err("Sale duration must not exceed maximum duration.");
			}

			if input.sale.initial_weight < input.sale.final_weight {
				return Err("Initial weight must be greater than final weight.");
			}

			if input.sale.initial_weight > T::MaxInitialWeight::get() {
				return Err("Initial weight must not exceed the defined maximum.");
			}

			if input.sale.final_weight < T::MinFinalWeight::get() {
				return Err("Final weight must not be lower than the defined minimum.");
			}

			Ok(input)
		}
	}

	type AssetIdOf<T> = <T as Config>::AssetId;
	type BalanceOf<T> = <T as Config>::Balance;
	type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	type PoolIdOf<T> = <T as Config>::PoolId;
	type PoolOf<T> = Pool<AccountIdOf<T>, BlockNumberFor<T>, AssetIdOf<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Pool with specified id `T::PoolId` was created successfully by `T::AccountId`.
		PoolCreated {
			/// Id of newly created pool.
			pool_id: T::PoolId,
			/// Owner of the pool.
			owner: T::AccountId,
		},
		/// The sale ended, the funds repatriated and the pool deleted.
		PoolDeleted {
			/// Pool that was removed.
			pool_id: T::PoolId,
			/// Owner of the pool that we repatriated the funds to.
			owner: T::AccountId,
			/// Amount of base asset repatriated.
			base_amount: T::Balance,
			/// Amount of quote asset repatriated.
			quote_amount: T::Balance,
		},
		/// Liquidity added into the pool `T::PoolId`.
		LiquidityAdded {
			/// Pool id to which liquidity added.
			pool_id: T::PoolId,
			/// Amount of base asset deposited.
			base_amount: T::Balance,
			/// Amount of quote asset deposited.
			quote_amount: T::Balance,
		},
		/// Token exchange happened.
		Swapped {
			/// Pool id on which exchange done.
			pool_id: T::PoolId,
			/// Account id who exchanged token.
			who: T::AccountId,
			/// Id of asset used as input.
			base_asset: T::AssetId,
			/// Id of asset used as output.
			quote_asset: T::AssetId,
			/// Amount of base asset received.
			base_amount: T::Balance,
			/// Amount of quote asset provided.
			quote_amount: T::Balance,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		PoolNotFound,
		PairMismatch,
		MustBeOwner,
		InvalidSaleState,
		InvalidAmount,
		CannotRespectMinimumRequested,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[allow(missing_docs)]
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Type representing the unique ID of an asset.
		type AssetId: FullCodec
			+ MaxEncodedLen
			+ Eq
			+ PartialEq
			+ Copy
			+ Clone
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default
			+ TypeInfo
			+ Ord;

		/// Type representing the Balance of an account.
		type Balance: Default
			+ Parameter
			+ Codec
			+ MaxEncodedLen
			+ Copy
			+ Ord
			+ Zero
			+ SafeArithmetic;

		/// An isomorphism: Balance<->u128
		type Convert: Convert<u128, BalanceOf<Self>> + Convert<BalanceOf<Self>, u128>;

		/// Dependency allowing this pallet to transfer funds from one account to another.
		type Assets: Transfer<AccountIdOf<Self>, Balance = BalanceOf<Self>, AssetId = AssetIdOf<Self>>
			+ Inspect<AccountIdOf<Self>, Balance = BalanceOf<Self>, AssetId = AssetIdOf<Self>>;

		/// Type representing the unique ID of a pool.
		type PoolId: FullCodec
			+ MaxEncodedLen
			+ Default
			+ Debug
			+ TypeInfo
			+ Eq
			+ PartialEq
			+ Ord
			+ Copy
			+ Zero
			+ One
			+ SafeArithmetic;

		type LocalAssets: LocalAssets<AssetIdOf<Self>>;

		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// Maximum duration for a sale.
		#[pallet::constant]
		type MaxSaleDuration: Get<BlockNumberFor<Self>>;

		/// Maximum initial weight.
		#[pallet::constant]
		type MaxInitialWeight: Get<Permill>;

		/// Minimum final weight.
		#[pallet::constant]
		type MinFinalWeight: Get<Permill>;

		type AdminOrigin: EnsureOrigin<Self::Origin>;

		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::type_value]
	pub fn PoolCountOnEmpty<T: Config>() -> T::PoolId {
		Zero::zero()
	}

	#[pallet::storage]
	#[pallet::getter(fn pool_count)]
	#[allow(clippy::disallowed_type)]
	pub type PoolCount<T: Config> = StorageValue<_, T::PoolId, ValueQuery, PoolCountOnEmpty<T>>;

	#[pallet::storage]
	#[pallet::getter(fn pools)]
	pub type Pools<T: Config> = StorageMap<_, Blake2_128Concat, T::PoolId, PoolOf<T>>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new pool.
		///
		/// Emits `PoolCreated` even when successful.
		#[pallet::weight(T::WeightInfo::create())]
		pub fn create(
			origin: OriginFor<T>,
			pool: Validated<PoolOf<T>, PoolIsValid<T>>,
		) -> DispatchResult {
			let _ = T::AdminOrigin::ensure_origin(origin)?;
			let _ = Self::do_create_pool(pool)?;
			Ok(())
		}

		/// Execute a buy order on a pool.
		///
		/// The `base_amount` always represent the base asset amount (A/B => A).
		///
		/// Emits `Swapped` event when successful.
		#[pallet::weight(T::WeightInfo::buy())]
		pub fn buy(
			origin: OriginFor<T>,
			pool_id: T::PoolId,
			asset_id: T::AssetId,
			amount: T::Balance,
			keep_alive: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let _ = <Self as CurveAmm>::buy(&who, pool_id, asset_id, amount, keep_alive)?;
			Ok(())
		}

		/// Execute a sell order on a pool.
		///
		/// The `base_amount` always represent the base asset amount (A/B => A).
		///
		/// Emits `Swapped` event when successful.
		#[pallet::weight(T::WeightInfo::sell())]
		pub fn sell(
			origin: OriginFor<T>,
			pool_id: T::PoolId,
			asset_id: T::AssetId,
			amount: T::Balance,
			keep_alive: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let _ = <Self as CurveAmm>::sell(&who, pool_id, asset_id, amount, keep_alive)?;
			Ok(())
		}

		/// Execute a specific swap operation.
		///
		/// Buy operation if the pair is the original pool pair (A/B).
		/// Sell operation if the pair is the original pool pair swapped (B/A).
		///
		/// The `quote_amount` is always the quote asset amount (A/B => B), (B/A => A).
		///
		/// Emits `Swapped` event when successful.
		#[pallet::weight(T::WeightInfo::swap())]
		pub fn swap(
			origin: OriginFor<T>,
			pool_id: T::PoolId,
			pair: CurrencyPair<T::AssetId>,
			quote_amount: T::Balance,
			min_receive: T::Balance,
			keep_alive: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let _ = <Self as CurveAmm>::exchange(
				&who,
				pool_id,
				pair,
				quote_amount,
				min_receive,
				keep_alive,
			)?;
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub(crate) fn do_create_pool(
			pool: Validated<PoolOf<T>, PoolIsValid<T>>,
		) -> Result<T::PoolId, DispatchError> {
			let pool_id =
				PoolCount::<T>::try_mutate(|pool_count| -> Result<T::PoolId, DispatchError> {
					let pool_id = *pool_count;
					Pools::<T>::insert(pool_id, pool.clone().value());
					*pool_count = pool_id.safe_add(&T::PoolId::one())?;
					Ok(pool_id)
				})?;

			Self::deposit_event(Event::PoolCreated { pool_id, owner: pool.owner.clone() });

			Ok(pool_id)
		}

		pub(crate) fn get_pool_ensuring_sale_state(
			pool_id: T::PoolId,
			current_block: BlockNumberFor<T>,
			expected_sale_state: SaleState,
		) -> Result<PoolOf<T>, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			ensure!(
				pool.sale.state(current_block) == expected_sale_state,
				Error::<T>::InvalidSaleState
			);
			Ok(pool)
		}

		pub(crate) fn get_pool(pool_id: T::PoolId) -> Result<PoolOf<T>, DispatchError> {
			Pools::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound.into())
		}

		pub(crate) fn account_id(pool_id: &T::PoolId) -> T::AccountId {
			T::PalletId::get().into_sub_account(pool_id)
		}

		pub(crate) fn do_spot_price(
			pool_id: T::PoolId,
			pair: CurrencyPair<AssetIdOf<T>>,
			current_block: BlockNumberFor<T>,
		) -> Result<BalanceOf<T>, DispatchError> {
			let pool =
				Self::get_pool_ensuring_sale_state(pool_id, current_block, SaleState::Ongoing)?;
			ensure!(pair == pool.pair, Error::<T>::PairMismatch);

			let weights = pool.sale.current_weights(current_block)?;

			/*
			https://balancer.fi/whitepaper.pdf, (15)
				  ai = amount in = quote_amount
				  ao = amount out = base_amount
				  bi = balance in = quote asset balance
				  bo = balance out = base asset balance
			wi = weight in
			wo = weight out
				   */
			let (wo, wi) =
				if pair.base == pool.pair.base { weights } else { (weights.1, weights.0) };

			let pool_account = Self::account_id(&pool_id);
			let bi = Decimal::from_u128(T::Convert::convert(T::Assets::balance(
				pair.quote,
				&pool_account,
			)))
			.ok_or(ArithmeticError::Overflow)?;
			let bo = Decimal::from_u128(T::Convert::convert(T::Assets::balance(
				pair.base,
				&pool_account,
			)))
			.ok_or(ArithmeticError::Overflow)?;

			let full_percent =
				Decimal::from_u32(Permill::one().deconstruct()).ok_or(ArithmeticError::Overflow)?;
			let wi_numer = Decimal::from_u32(wi.deconstruct()).ok_or(ArithmeticError::Overflow)?;
			let wi = wi_numer.safe_div(&full_percent)?;
			let wo_numer = Decimal::from_u32(wo.deconstruct()).ok_or(ArithmeticError::Overflow)?;
			let wo = wo_numer.safe_div(&full_percent)?;
			let bi_div_wi = bi.safe_div(&wi)?;
			let bo_div_wo = bo.safe_div(&wo)?;
			let spot_price = bi_div_wi.safe_div(&bo_div_wo)?;

			let base_unit = Decimal::from_u128(T::LocalAssets::unit::<u128>(pair.base)?)
				.ok_or(ArithmeticError::Overflow)?;

			Ok(T::Convert::convert(
				spot_price.safe_mul(&base_unit)?.to_u128().ok_or(ArithmeticError::Overflow)?,
			))
		}

		pub(crate) fn do_get_exchange(
			pool_id: T::PoolId,
			pair: CurrencyPair<AssetIdOf<T>>,
			current_block: BlockNumberFor<T>,
			quote_amount: BalanceOf<T>,
		) -> Result<BalanceOf<T>, DispatchError> {
			let pool =
				Self::get_pool_ensuring_sale_state(pool_id, current_block, SaleState::Ongoing)?;

			ensure!(pair == pool.pair, Error::<T>::PairMismatch);
			ensure!(!quote_amount.is_zero(), Error::<T>::InvalidAmount);

			let weights = pool.sale.current_weights(current_block)?;

			/*
			https://balancer.fi/whitepaper.pdf, (15)
				  ai = amount in = quote_amount
				  ao = amount out = base_amount
				  bi = balance in = quote asset balance
				  bo = balance out = base asset balance
			wi = weight in
			wo = weight out
				   */
			let (wo, wi) =
				if pair.base == pool.pair.base { weights } else { (weights.1, weights.0) };

			let pool_account = Self::account_id(&pool_id);

			let ai = Decimal::from_u128(T::Convert::convert(quote_amount))
				.ok_or(ArithmeticError::Overflow)?;
			let bi = Decimal::from_u128(T::Convert::convert(T::Assets::balance(
				pair.quote,
				&pool_account,
			)))
			.ok_or(ArithmeticError::Overflow)?;
			let bo = Decimal::from_u128(T::Convert::convert(T::Assets::balance(
				pair.base,
				&pool_account,
			)))
			.ok_or(ArithmeticError::Overflow)?;
			let weight_numer =
				Decimal::from_u32(wi.deconstruct()).ok_or(ArithmeticError::Overflow)?;
			let weight_denom =
				Decimal::from_u32(wo.deconstruct()).ok_or(ArithmeticError::Overflow)?;
			let weight_power = weight_numer.safe_div(&weight_denom)?;
			let bi_div_bi_plus_ai = bi.safe_div(&bi.safe_add(&ai)?)?;
			let term_to_weight_power =
				bi_div_bi_plus_ai.checked_powd(weight_power).ok_or(ArithmeticError::Overflow)?;
			let one_minus_term = Decimal::one().safe_sub(&term_to_weight_power)?;
			let ao = bo.safe_mul(&one_minus_term)?.to_u128().ok_or(ArithmeticError::Overflow)?;
			let base_amount = T::Convert::convert(ao);

			Ok(base_amount)
		}
	}

	impl<T: Config> CurveAmm for Pallet<T> {
		type AssetId = AssetIdOf<T>;
		type Balance = BalanceOf<T>;
		type AccountId = AccountIdOf<T>;
		type PoolId = PoolIdOf<T>;

		fn pool_exists(pool_id: Self::PoolId) -> bool {
			Pools::<T>::contains_key(pool_id)
		}

		fn currency_pair(
			pool_id: Self::PoolId,
		) -> Result<CurrencyPair<Self::AssetId>, DispatchError> {
			Ok(Self::get_pool(pool_id)?.pair)
		}

		fn get_exchange_value(
			pool_id: Self::PoolId,
			asset_id: Self::AssetId,
			amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			let pair = if asset_id == pool.pair.base { pool.pair } else { pool.pair.swap() };
			let current_block = frame_system::Pallet::<T>::current_block_number();
			Self::do_get_exchange(pool_id, pair, current_block, amount)
		}

		#[transactional]
		fn buy(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			asset_id: Self::AssetId,
			amount: Self::Balance,
			keep_alive: bool,
		) -> Result<Self::Balance, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			let pair = if asset_id == pool.pair.base { pool.pair.swap() } else { pool.pair };
			let quote_amount = Self::get_exchange_value(pool_id, asset_id, amount)?;
			<Self as CurveAmm>::exchange(
				who,
				pool_id,
				pair,
				quote_amount,
				T::Balance::zero(),
				keep_alive,
			)
		}

		#[transactional]
		fn sell(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			asset_id: Self::AssetId,
			amount: Self::Balance,
			keep_alive: bool,
		) -> Result<Self::Balance, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			let pair = if asset_id == pool.pair.base { pool.pair } else { pool.pair.swap() };
			<Self as CurveAmm>::exchange(who, pool_id, pair, amount, T::Balance::zero(), keep_alive)
		}

		#[transactional]
		fn add_liquidity(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			base_amount: Self::Balance,
			quote_amount: Self::Balance,
			_: Self::Balance,
			keep_alive: bool,
		) -> Result<(), DispatchError> {
			let current_block = frame_system::Pallet::<T>::current_block_number();
			let pool =
				Self::get_pool_ensuring_sale_state(pool_id, current_block, SaleState::NotStarted)?;

			ensure!(pool.owner == *who, Error::<T>::MustBeOwner);
			ensure!(!base_amount.is_zero() && !quote_amount.is_zero(), Error::<T>::InvalidAmount);

			// NOTE(hussein-aitlahcen): as we only allow the owner to provide liquidity, we don't mint any LP.
			let pool_account = Self::account_id(&pool_id);
			T::Assets::transfer(pool.pair.base, who, &pool_account, base_amount, keep_alive)?;
			T::Assets::transfer(pool.pair.quote, who, &pool_account, quote_amount, keep_alive)?;
			Ok(())
		}

		#[transactional]
		fn remove_liquidity(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			_: Self::Balance,
			_: Self::Balance,
			_: Self::Balance,
		) -> Result<(), DispatchError> {
			let current_block = frame_system::Pallet::<T>::current_block_number();
			let pool =
				Self::get_pool_ensuring_sale_state(pool_id, current_block, SaleState::Ended)?;

			ensure!(pool.owner == *who, Error::<T>::MustBeOwner);

			let pool_account = Self::account_id(&pool_id);

			let repatriate = |a| -> Result<BalanceOf<T>, DispatchError> {
				let a_balance = T::Assets::balance(a, &pool_account);
				// NOTE(hussein-aitlahcen): not need to keep the pool account alive.
				T::Assets::transfer(a, &pool_account, who, a_balance, false)?;
				Ok(a_balance)
			};

			let base_amount = repatriate(pool.pair.base)?;
			let quote_amount = repatriate(pool.pair.quote)?;

			Pools::<T>::remove(pool_id);

			Self::deposit_event(Event::PoolDeleted {
				pool_id,
				owner: pool.owner,
				base_amount,
				quote_amount,
			});

			Ok(())
		}

		#[transactional]
		fn exchange(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			pair: CurrencyPair<Self::AssetId>,
			quote_amount: Self::Balance,
			min_receive: Self::Balance,
			keep_alive: bool,
		) -> Result<Self::Balance, DispatchError> {
			let current_block = frame_system::Pallet::<T>::current_block_number();
			let base_amount = Self::do_get_exchange(pool_id, pair, current_block, quote_amount)?;

			ensure!(base_amount >= min_receive, Error::<T>::CannotRespectMinimumRequested);

			let pool_account = Self::account_id(&pool_id);
			T::Assets::transfer(pair.quote, who, &pool_account, quote_amount, keep_alive)?;
			// NOTE(hussein-aitlance): no need to keep alive the pool account
			T::Assets::transfer(pair.base, &pool_account, who, base_amount, false)?;

			Self::deposit_event(Event::Swapped {
				pool_id,
				who: who.clone(),
				base_asset: pair.base,
				quote_asset: pair.quote,
				base_amount,
				quote_amount,
			});

			Ok(base_amount)
		}
	}
}
