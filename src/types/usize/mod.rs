pub mod partial_eq;
pub mod default;
pub mod deref;
pub mod from;

pub mod add_assign_self;
pub mod sub_assign_self;
pub mod mul_assign_self;

pub mod div_assign_non_zero;

#[cfg(target_pointer_width = "32")] mod if_32;
#[cfg(target_pointer_width = "64")] mod if_64;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, Eq)]
#[repr(transparent)]
pub struct Bound_usize<const MIN: usize, const MAX: usize> {
	inner: usize,
}

impl<const MIN: usize, const MAX: usize> Bound_usize<MIN, MAX> {
	const OK: () = assert!(MIN <= MAX, "MIN must be less than or equal to MAX");
	
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	pub const fn new(mut inner: usize) -> Self {
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
	pub const fn get(&self) -> usize {
		return self.inner;
	}

	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	pub fn set(&mut self, mut inner: usize) {
		if inner < MIN {
			inner = MIN;
		} else if inner > MAX {
			inner = MAX;
		}

		self.inner = inner;
	}

	pub const fn bound_lower() -> usize { MIN }
	pub const fn bound_upper() -> usize { MAX }
}

#[cfg(test)]
pub mod tests {
	use crate::prelude::Bound_usize;
	use crate::types::test_macros::new_set::{test_unsigned_new, test_unsigned_set};

	#[test] fn test_new() { test_unsigned_new!(Bound_usize); }
	#[test] fn test_set() { test_unsigned_set!(Bound_usize); }
	#[test] fn test_serde() { crate::types::test_macros::serde::test_unsigned_serde!(Bound_usize); }
}