pub mod partial_eq;
pub mod default;
pub mod deref;
pub mod from;

pub mod add_assign_self;
pub mod sub_assign_self;
pub mod mul_assign_self;

pub mod add_assign_squeeze;
pub mod sub_assign_squeeze;
pub mod mul_assign_squeeze;

#[cfg(feature = "div_assign_zero")] pub mod div_assign_zero;
pub mod div_assign_non_zero;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, Eq)]
#[repr(transparent)]
pub struct Bound_u64<const MIN: u64, const MAX: u64> {
	inner: u64,
}

impl<const MIN: u64, const MAX: u64> Bound_u64<MIN, MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	pub fn new(mut inner: u64) -> Self {

		if inner < MIN {
			inner = MIN;
		} else if inner > MAX {
			inner = MAX;
		}

		return Self { inner };
	}

	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	pub fn get(&self) -> u64 {
		return self.inner;
	}

	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	pub fn set(&mut self, mut inner: u64) {
		if inner < MIN {
			inner = MIN;
		} else if inner > MAX {
			inner = MAX;
		}

		self.inner = inner;
	}

	pub fn bound_lower() -> u64 { MIN }
	pub fn bound_upper() -> u64 { MAX }
}

#[cfg(test)]
pub mod tests {
	use crate::prelude::Bound_u64;
	use crate::types::test_macros::new_set::{test_unsigned_new, test_unsigned_set};
	use crate::types::test_macros::serde::test_unsigned_serde;

	#[test] fn test_new() { test_unsigned_new!(Bound_u64); }
	#[test] fn test_set() { test_unsigned_set!(Bound_u64); }
	#[test] fn test_serde() { test_unsigned_serde!(Bound_u64); }
}