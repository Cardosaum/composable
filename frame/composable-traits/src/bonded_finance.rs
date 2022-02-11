use crate::math::SafeArithmetic;
use composable_support::validation::{Validate, Validated};
use frame_support::{pallet_prelude::*, traits::Get};
use scale_info::TypeInfo;
use sp_runtime::{traits::Zero, ArithmeticError};

pub trait BondedFinance {
	type AccountId;
	type AssetId;
	type Balance;
	type BlockNumber;
	type BondOfferId;
	type MinReward;
	type MinVestedTransfer;

	/// Create a new offer.
	fn offer(
		from: &Self::AccountId,
		offer: Validated<
			BondOffer<Self::AccountId, Self::AssetId, Self::Balance, Self::BlockNumber>,
			ValidBondOffer<Self::MinReward, Self::MinVestedTransfer>,
		>,
		keep_alive: bool,
	) -> Result<Self::BondOfferId, DispatchError>;

	/// Bond for an offer.
	fn bond(
		offer: Self::BondOfferId,
		from: &Self::AccountId,
		nb_of_bonds: Self::Balance,
		keep_alive: bool,
	) -> Result<Self::Balance, DispatchError>;
}

/// The Bond duration.
#[derive(Clone, Encode, Decode, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
pub enum BondDuration<BlockNumber> {
	/// Finite duration, liquidity is returned after a number of `blocks`.
	Finite { return_in: BlockNumber },
	/// Infinite duration, the protocol is now owning the liquidity
	Infinite,
}

/// The Bond offer.
#[derive(Clone, Encode, Decode, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
pub struct BondOffer<AccountId, AssetId, Balance, BlockNumber> {
	/// The account that will receive the locked assets.
	pub beneficiary: AccountId,
	/// Asset to be locked. Unlockable after `duration`.
	/// Asset which `beneficiary` wants to get for his offer.
	pub asset: AssetId,
	/// Price of a bond unit in `asset`.
	pub bond_price: Balance,
	/// Number of bonds. We use the Balance type for the sake of simplicity.
	pub nb_of_bonds: Balance,
	/// Duration for which the asset has to be locked.
	pub maturity: BondDuration<BlockNumber>,
	/// Total reward for this offer.
	pub reward: BondOfferReward<AssetId, Balance, BlockNumber>,
}

/// The Bond reward. Asset and rules reward will be given.
#[derive(Clone, Encode, Decode, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
pub struct BondOfferReward<AssetId, Balance, BlockNumber> {
	/// The actual reward asset.
	pub asset: AssetId,
	/// Total reward.
	pub amount: Balance,
	/// Duration after which the reward can be claimed.
	pub maturity: BlockNumber,
}

#[derive(Debug, Decode)]
pub struct ValidBondOffer<U, V> {
	_m1: PhantomData<U>,
	_m2: PhantomData<V>,
}

impl<U, V> Copy for ValidBondOffer<U, V> {}

impl<U, V> Clone for ValidBondOffer<U, V> {
	fn clone(&self) -> Self {
		*self
	}
}

impl<
		MinTransfer,
		MinReward,
		AccountId,
		AssetId,
		Balance: Zero + PartialOrd + SafeArithmetic,
		BlockNumber: Zero,
	>
	Validate<
		BondOffer<AccountId, AssetId, Balance, BlockNumber>,
		ValidBondOffer<MinTransfer, MinReward>,
	> for ValidBondOffer<MinTransfer, MinReward>
where
	ValidBondOffer<MinTransfer, MinReward>: Decode,
	MinTransfer: Get<Balance>,
	MinReward: Get<Balance>,
{
	fn validate(
		input: BondOffer<AccountId, AssetId, Balance, BlockNumber>,
	) -> Result<BondOffer<AccountId, AssetId, Balance, BlockNumber>, &'static str> {
		let nonzero_maturity = match &input.maturity {
			BondDuration::Finite { return_in } => !return_in.is_zero(),
			BondDuration::Infinite => true,
		};

		if nonzero_maturity == false {
			return Err("invalid maturity")
		}

		if input.bond_price < MinTransfer::get() {
			return Err("invalid bond_price")
		}

		if input.nb_of_bonds.is_zero() {
			return Err("invalid nb_of_bonds")
		}

		let valid_reward = input.reward.amount >= MinReward::get() &&
			input
				.reward
				.amount
				.safe_div(&input.nb_of_bonds)
				.unwrap_or_else(|_| Balance::zero()) >=
				MinTransfer::get();

		if !valid_reward {
			return Err("invalid reward")
		}

		if input.reward.maturity.is_zero() {
			return Err("invalid reward_maturity")
		}

		if !input.total_price().is_ok() {
			return Err("invalid total_price")
		}

		Ok(input)
	}
}

impl<AccountId, AssetId, Balance: Zero + PartialOrd + SafeArithmetic, BlockNumber: Zero>
	BondOffer<AccountId, AssetId, Balance, BlockNumber>
{
	/// An offer is completed once all it's nb_of_bonds has been sold.
	pub fn completed(&self) -> bool {
		self.nb_of_bonds.is_zero()
	}
	/// The total price of the offer, which is the number of nb_of_bonds * the bond_price.
	pub fn total_price(&self) -> Result<Balance, ArithmeticError> {
		self.nb_of_bonds.safe_mul(&self.bond_price)
	}
	/// Check whether an offer is valid and can be submitted.
	pub fn valid(&self, min_transfer: Balance, min_reward: Balance) -> bool {
		let nonzero_maturity = match &self.maturity {
			BondDuration::Finite { return_in } => !return_in.is_zero(),
			BondDuration::Infinite => true,
		};
		let valid_price = self.bond_price >= min_transfer;
		let nonzero_nb_of_bonds = !self.nb_of_bonds.is_zero();
		let valid_reward = self.reward.amount >= min_reward &&
			self.reward
				.amount
				.safe_div(&self.nb_of_bonds)
				.unwrap_or_else(|_| Balance::zero()) >=
				min_transfer;
		let nonzero_reward_maturity = !self.reward.maturity.is_zero();
		let valid_total = self.total_price().is_ok();
		nonzero_maturity &&
			nonzero_nb_of_bonds &&
			valid_price && nonzero_reward_maturity &&
			valid_reward && valid_total
	}
}
