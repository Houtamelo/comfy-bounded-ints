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

pub mod div_assign_non_zero;
#[cfg(feature = "div_assign_zero")] pub mod div_assign_zero;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, Eq)]
#[repr(transparent)]
pub struct Bound_i16<const MIN: i16, const MAX: i16> {
	inner: i16,
}

impl<const MIN: i16, const MAX: i16> Bound_i16<MIN, MAX> {
	const OK: () = assert!(MIN <= MAX, "MIN must be less than or equal to MAX");
	
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	pub const fn new(mut inner: i16) -> Self {
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
	pub const fn get(&self) -> i16 {
		return self.inner;
	}

	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	pub fn set(&mut self, mut inner: i16) {
		if inner < MIN {
			inner = MIN;
		} else if inner > MAX {
			inner = MAX;
		}

		self.inner = inner;
	}

	pub const fn bound_lower() -> i16 { MIN }
	pub const fn bound_upper() -> i16 { MAX }
}

#[cfg(test)]
pub mod tests {
	use crate::prelude::Bound_i16;
	use crate::types::test_macros::new_set::{test_signed_new, test_signed_set};

	#[test] fn test_new() { test_signed_new!(Bound_i16); }
	#[test] fn test_set() { test_signed_set!(Bound_i16); }
	#[test] fn test_serde() { crate::types::test_macros::serde::test_signed_serde!(Bound_i16); }
}