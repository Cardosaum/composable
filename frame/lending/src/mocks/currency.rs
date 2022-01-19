use crate::{self as pallet_lending, *};
use composable_traits::{
	currency::DynamicCurrencyId,
	defi::DeFiComposableConfig,
	governance::{GovernanceRegistry, SignedRawOrigin},
};
use frame_support::{
	ord_parameter_types, parameter_types,
	traits::{Everything, OnFinalize, OnInitialize},
	PalletId,
};
use frame_system::EnsureSignedBy;
use hex_literal::hex;
use once_cell::sync::Lazy;
use orml_traits::{parameter_type_with_key, GetByKey};
use scale_info::TypeInfo;
use sp_arithmetic::traits::Zero;
use sp_core::{sr25519::Signature, H256};
use sp_runtime::{
	testing::{Header, TestXt},
	traits::{
		BlakeTwo256, ConvertInto, Extrinsic as ExtrinsicT, IdentifyAccount, IdentityLookup, Verify,
	},
	ArithmeticError, DispatchError,
};


#[derive(
	PartialOrd,
	Ord,
	PartialEq,
	Eq,
	Debug,
	Copy,
	Clone,
	codec::Encode,
	codec::Decode,
	serde::Serialize,
	serde::Deserialize,
	TypeInfo,
)]
#[allow(clippy::upper_case_acronyms)] // currencies should be CONSTANT_CASE
pub enum MockCurrencyId {
	PICA,
	BTC,
	ETH,
	LTC,
	USDT,
	LpToken(u128),
}

impl From<u128> for MockCurrencyId {
	fn from(id: u128) -> Self {
		match id {
			0 => MockCurrencyId::PICA,
			1 => MockCurrencyId::BTC,
			2 => MockCurrencyId::ETH,
			3 => MockCurrencyId::LTC,
			4 => MockCurrencyId::USDT,
			5 => MockCurrencyId::LpToken(0),
			_ => unreachable!(),
		}
	}
}

impl Default for MockCurrencyId {
	fn default() -> Self {
		MockCurrencyId::PICA
	}
}

impl DynamicCurrencyId for MockCurrencyId {
	fn next(self) -> Result<Self, DispatchError> {
		match self {
			MockCurrencyId::LpToken(x) => Ok(MockCurrencyId::LpToken(
				x.checked_add(1).ok_or(DispatchError::Arithmetic(ArithmeticError::Overflow))?,
			)),
			_ => unreachable!(),
		}
	}
}