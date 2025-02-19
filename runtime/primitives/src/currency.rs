//! CurrencyId implementation
use codec::{CompactAs, Decode, Encode, MaxEncodedLen};
use composable_traits::currency::Exponent;
use core::ops::Div;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

use composable_support::rpc_helpers::FromHexStr;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::sp_std::ops::Deref;

#[derive(
	Encode,
	Decode,
	MaxEncodedLen,
	Eq,
	PartialEq,
	Copy,
	Clone,
	RuntimeDebug,
	PartialOrd,
	Ord,
	TypeInfo,
	CompactAs,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct CurrencyId(pub u128);

impl CurrencyId {
	pub const INVALID: CurrencyId = CurrencyId(0);
	pub const PICA: CurrencyId = CurrencyId(1);
	pub const LAYR: CurrencyId = CurrencyId(2);
	pub const CROWD_LOAN: CurrencyId = CurrencyId(3);
	pub const KSM: CurrencyId = CurrencyId(4);

	#[inline(always)]
	pub fn decimals(&self) -> Exponent {
		12
	}
	pub fn unit<T: From<u64>>(&self) -> T {
		T::from(10_u64.pow(self.decimals()))
	}
	pub fn milli<T: From<u64> + Div<Output = T>>(&self) -> T {
		self.unit::<T>() / T::from(1000_u64)
	}
}

impl FromHexStr for CurrencyId {
	type Err = <u128 as FromHexStr>::Err;

	fn from_hex_str(src: &str) -> core::result::Result<Self, Self::Err> {
		u128::from_hex_str(src).map(CurrencyId)
	}
}

impl core::fmt::LowerHex for CurrencyId {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		core::fmt::LowerHex::fmt(&self.0, f)
	}
}

impl Default for CurrencyId {
	#[inline]
	fn default() -> Self {
		CurrencyId::INVALID
	}
}

impl Deref for CurrencyId {
	type Target = u128;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl From<CurrencyId> for u128 {
	#[inline]
	fn from(id: CurrencyId) -> Self {
		id.0
	}
}

impl From<u128> for CurrencyId {
	#[inline]
	fn from(raw: u128) -> Self {
		CurrencyId(raw)
	}
}

/// maps id to junction generic key,
/// unfortunately it is the best way to encode currency id as of now in XCM
#[cfg(feature = "develop")]
impl From<CurrencyId> for xcm::latest::Junction {
	fn from(this: CurrencyId) -> Self {
		xcm::latest::Junction::GeneralKey(this.encode())
	}
}

mod ops {
	use super::CurrencyId;
	use core::ops::{Add, Mul};
	use sp_runtime::traits::{Bounded, CheckedAdd, CheckedMul, One, Saturating, Zero};

	impl Add for CurrencyId {
		type Output = Self;

		fn add(self, rhs: Self) -> Self::Output {
			CurrencyId(self.0.add(rhs.0))
		}
	}

	impl Mul for CurrencyId {
		type Output = CurrencyId;

		fn mul(self, rhs: Self) -> Self::Output {
			CurrencyId(self.0.mul(rhs.0))
		}
	}

	impl CheckedAdd for CurrencyId {
		fn checked_add(&self, v: &Self) -> Option<Self> {
			Some(CurrencyId(self.0.checked_add(v.0)?))
		}
	}

	impl CheckedMul for CurrencyId {
		fn checked_mul(&self, v: &Self) -> Option<Self> {
			Some(CurrencyId(self.0.checked_mul(v.0)?))
		}
	}

	impl Zero for CurrencyId {
		fn zero() -> Self {
			CurrencyId(0)
		}

		fn is_zero(&self) -> bool {
			self.0.is_zero()
		}
	}

	impl One for CurrencyId {
		fn one() -> Self {
			CurrencyId(u128::one())
		}
	}

	impl Bounded for CurrencyId {
		fn min_value() -> Self {
			CurrencyId(u128::min_value())
		}

		fn max_value() -> Self {
			CurrencyId(u128::max_value())
		}
	}

	impl Saturating for CurrencyId {
		fn saturating_add(self, rhs: Self) -> Self {
			self.0.saturating_add(rhs.0).into()
		}

		fn saturating_sub(self, rhs: Self) -> Self {
			<u128 as Saturating>::saturating_sub(self.0, rhs.0).into()
		}

		fn saturating_mul(self, rhs: Self) -> Self {
			<u128 as Saturating>::saturating_mul(self.0, rhs.0).into()
		}

		fn saturating_pow(self, exp: usize) -> Self {
			<u128 as Saturating>::saturating_pow(self.0, exp).into()
		}
	}
}
