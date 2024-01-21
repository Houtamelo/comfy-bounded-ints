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
pub struct Bound_u32<const MIN: u32, const MAX: u32> {
	inner: u32,
}

impl<const MIN: u32, const MAX: u32> Bound_u32<MIN, MAX> {
	const OK: () = assert!(MIN <= MAX, "MIN must be less than or equal to MAX");
	
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	pub const fn new(mut inner: u32) -> Self {
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
	pub const fn get(&self) -> u32 {
		return self.inner;
	}

	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	pub fn set(&mut self, mut inner: u32) {
		if inner < MIN {
			inner = MIN;
		} else if inner > MAX {
			inner = MAX;
		}

		self.inner = inner;
	}

	pub const fn bound_lower() -> u32 { MIN }
	pub const fn bound_upper() -> u32 { MAX }
}

#[cfg(test)]
pub mod tests {
	use crate::prelude::Bound_u32;
	use crate::types::test_macros::new_set::{test_unsigned_new, test_unsigned_set};

	#[test] fn test_new() { test_unsigned_new!(Bound_u32); }
	#[test] fn test_set() { test_unsigned_set!(Bound_u32); }
	#[test] fn test_serde() { crate::types::test_macros::serde::test_unsigned_serde!(Bound_u32); }
}