use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
use sp_std::vec::Vec;

/// An object that is able to tell us whether an entry can or cannot be disabled.
pub trait CallFilterHook {
	fn enable_hook(entry: &CallFilterEntry) -> DispatchResult;
	fn disable_hook(entry: &CallFilterEntry) -> DispatchResult;
}

impl CallFilterHook for () {
	#[inline(always)]
	fn enable_hook(_: &CallFilterEntry) -> DispatchResult {
		Ok(())
	}
	#[inline(always)]
	fn disable_hook(_: &CallFilterEntry) -> DispatchResult {
		Ok(())
	}
}

/// An object that is able to pause/unpause extrinsics.
pub trait CallFilter {
	fn disabled(entry: &CallFilterEntry) -> bool;
	fn enable(entry: &CallFilterEntry) -> DispatchResult;
	fn disable(entry: &CallFilterEntry) -> DispatchResult;
}

/// A call filter entry, product of the pallet name and the extrinsic name.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct CallFilterEntry {
	pub pallet_name: BoundedVec<u8, MaxBytes>,
	pub function_name: BoundedVec<u8, MaxBytes>,
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct MaxBytes {}

impl Get<u32> for MaxBytes {
	fn get() -> u32 {
		20000
	}
}

impl CallFilterEntry {
    pub fn new(pallet_name: &'static str, function_name: &'static str) -> Result<Self, ()> {
		Ok(Self {
			pallet_name: pallet_name.as_bytes().to_vec().try_into()?,
			function_name: function_name.as_bytes().to_vec().try_into()?,
		})
	}

	pub fn valid(&self) -> bool {
		sp_std::str::from_utf8(&self.pallet_name).is_ok() &&
			sp_std::str::from_utf8(&self.function_name).is_ok()
	}
}
