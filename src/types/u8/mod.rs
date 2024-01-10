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
pub struct Bound_u8<const MIN: u8, const MAX: u8> {
	inner: u8,
}

impl<const MIN: u8, const MAX: u8> Bound_u8<MIN, MAX> {
	const OK: () = assert!(MIN <= MAX, "MIN must be less than or equal to MAX");
	
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	pub fn new(mut inner: u8) -> Self {
		let _ = Self::OK; // this is not included in the binary
		
		if inner < MIN {
			inner = MIN;
		} else if inner > MAX {
			inner = MAX;
		}

		return Self { inner };
	}

	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	pub fn get(&self) -> u8 {
		return self.inner;
	}

	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	pub fn set(&mut self, mut inner: u8) {
		if inner < MIN {
			inner = MIN;
		} else if inner > MAX {
			inner = MAX;
		}

		self.inner = inner;
	}

	pub fn bound_lower() -> u8 { MIN }
	pub fn bound_upper() -> u8 { MAX }
}

#[cfg(test)]
pub mod tests {
	use crate::prelude::Bound_u8;
	use crate::types::test_macros::new_set::{test_unsigned_new, test_unsigned_set};
	
	#[test] fn test_new() { test_unsigned_new!(Bound_u8); }
	#[test] fn test_set() { test_unsigned_set!(Bound_u8); }
	#[test] fn test_serde() { crate::types::test_macros::serde::test_unsigned_serde!(Bound_u8); }
}