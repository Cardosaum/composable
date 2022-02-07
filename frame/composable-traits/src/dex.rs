use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{traits::Get, BoundedVec, RuntimeDebug};
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, FixedU128, Permill};
use sp_std::vec::Vec;

use crate::defi::CurrencyPair;

/// Implement AMM curve from "StableSwap - efficient mechanism for Stablecoin liquidity by Micheal
/// Egorov" Also blog at https://miguelmota.com/blog/understanding-stableswap-curve/ has very good explanation.
pub trait CurveAmm {
	/// The asset ID type
	type AssetId;
	/// The balance type of an account
	type Balance;
	/// The user account identifier type for the runtime
	type AccountId;

	/// Type that represents pool id
	type PoolId;

	/// Get currency_pair used in pool.
	fn currency_pair(pool_id: Self::PoolId) -> Result<CurrencyPair<Self::AssetId>, DispatchError>;

	/// Check pool with given id exists.
	fn pool_exists(pool_id: Self::PoolId) -> bool;

	/// Current number of pools (also ID for the next created pool)
	fn pool_count() -> Self::PoolId;

	/// Get pure exchange value for given units of given asset. (Note this does not include fees.)
	fn get_exchange_value(
		pool_id: Self::PoolId,
		asset_id: Self::AssetId,
		amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError>;

	/// Buy given `amount` of given asset from the pool.
	/// In buy user does not know how much assets he/she has to exchange to get desired amount.
	fn buy(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		asset_id: Self::AssetId,
		amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError>;

	/// Sell given `amount` of given asset to the pool.
	/// In sell user specifies `amount` of asset he/she wants to exchange to get other asset.
	fn sell(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		asset_id: Self::AssetId,
		amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError>;

	/// Deposit coins into the pool
	/// `amounts` - list of amounts of coins to deposit,
	/// `min_mint_amount` - minimum amout of LP tokens to mint from the deposit.
	fn add_liquidity(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		amounts: Vec<Self::Balance>,
		min_mint_amount: Self::Balance,
	) -> Result<(), DispatchError>;

	/// Withdraw coins from the pool.
	/// Withdrawal amount are based on current deposit ratios.
	/// `amount` - quantity of LP tokens to burn in the withdrawal,
	/// `min_amounts` - minimum amounts of underlying coins to receive.
	fn remove_liquidity(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		amount: Self::Balance,
		min_amounts: Vec<Self::Balance>,
	) -> Result<(), DispatchError>;

	/// Perform an exchange between two coins.
	/// `asset_id` - id of asset being exchanged.
	/// `dx` - amount of `i` being exchanged,
	/// `min_dy` - minimum amount of `j` to receive.
	fn exchange(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		asset_id: Self::AssetId,
		dx: Self::Balance,
		min_dy: Self::Balance,
	) -> Result<Self::Balance, DispatchError>;

	/// Withdraw admin fees
	fn withdraw_admin_fees(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		admin_fee_account: &Self::AccountId,
	) -> Result<(), DispatchError>;
}

/// Pool type
#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, Default, PartialEq, Eq, RuntimeDebug)]
pub struct StableSwapPoolInfo<AccountId, AssetId> {
	/// Owner of pool
	pub owner: AccountId,
	/// AssetId of LP token,
	pub lp_token: AssetId,
	/// Initial amplification coefficient
	pub amplification_coefficient: FixedU128,
	/// Amount of the fee pool charges for the exchange
	pub fee: Permill,
	/// Amount of the admin fee pool charges for the exchange
	pub admin_fee: Permill,
}

/// Describes a simple exchanges which does not allow advanced configurations such as slippage.
pub trait SimpleExchange {
	type AssetId;
	type Balance;
	type AccountId;
	type Error;

	/// Obtains the current price for a given asset, possibly routing through multiple markets.
	fn price(asset_id: Self::AssetId) -> Option<Self::Balance>;

	/// Exchange `amount` of `from` asset for `to` asset. The maximum price paid for the `to` asset
	/// is `SimpleExchange::price * (1 + slippage)`
	fn exchange(
		from: Self::AssetId,
		from_account: Self::AccountId,
		to: Self::AssetId,
		to_account: Self::AccountId,
		to_amount: Self::Balance,
		slippage: sp_runtime::Perbill,
	) -> Result<Self::Balance, DispatchError>;
}

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, Default, PartialEq, Eq, RuntimeDebug)]
pub struct ConstantProductPoolInfo<AccountId, AssetId> {
	/// Owner of pool
	pub owner: AccountId,
	/// AssetId of LP token,
	pub lp_token: AssetId,
	/// Amount of the fee pool charges for the exchange
	pub fee: Permill,
}

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum DexRouteNode<PoolId> {
	Curve(PoolId),
	Uniswap(PoolId),
}

/// Describes route for DEX.
/// `Direct` gives vector of pool_id to use as router.
#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum DexRoute<PoolId, MaxHops: Get<u32>> {
	Direct(BoundedVec<DexRouteNode<PoolId>, MaxHops>),
}

pub trait DexRouter<AccountId, AssetId, PoolId, Balance, MaxHops> {
	/// If route is `None` then delete existing entry for `asset_pair`
	/// If route is `Some` and no entry exist for `asset_pair` then add new entry
	/// else update existing entry.
	fn update_route(
		who: &AccountId,
		asset_pair: CurrencyPair<AssetId>,
		route: Option<BoundedVec<DexRouteNode<PoolId>, MaxHops>>,
	) -> Result<(), DispatchError>;
	/// If route exist return `Some(Vec<PoolId>)`, else `None`.
	fn get_route(asset_pair: CurrencyPair<AssetId>) -> Option<Vec<DexRouteNode<PoolId>>>;
	/// Exchange `dx` of `base` asset of `asset_pair` with associated route.
	fn exchange(
		who: &AccountId,
		asset_pair: CurrencyPair<AssetId>,
		dx: Balance,
	) -> Result<Balance, DispatchError>;
	/// Sell `amount` of `base` asset of asset_pair with associated route.
	fn sell(
		who: &AccountId,
		asset_pair: CurrencyPair<AssetId>,
		amount: Balance,
	) -> Result<Balance, DispatchError>;
	/// Buy `amount` of `quote` asset of asset_pair with associated route.
	fn buy(
		who: &AccountId,
		asset_pair: CurrencyPair<AssetId>,
		amount: Balance,
	) -> Result<Balance, DispatchError>;
}
