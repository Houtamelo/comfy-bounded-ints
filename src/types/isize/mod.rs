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
pub struct Bound_isize<const MIN: isize, const MAX: isize> {
	inner: isize,
}

impl<const MIN: isize, const MAX: isize> Bound_isize<MIN, MAX> {
	const OK: () = assert!(MIN <= MAX, "MIN must be less than or equal to MAX");
	
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	pub fn new(mut inner: isize) -> Self {
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
	pub fn get(&self) -> isize {
		return self.inner;
	}

	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	pub fn set(&mut self, mut inner: isize) {
		if inner < MIN {
			inner = MIN;
		} else if inner > MAX {
			inner = MAX;
		}

		self.inner = inner;
	}

	pub fn bound_lower() -> isize { MIN }
	pub fn bound_upper() -> isize { MAX }
}

#[cfg(test)]
pub mod tests {
	use crate::prelude::Bound_isize;
	use crate::types::test_macros::new_set::{test_signed_new, test_signed_set};

	#[test] fn test_new() { test_signed_new!(Bound_isize); }
	#[test] fn test_set() { test_signed_set!(Bound_isize); }
	#[test] fn test_serde() { crate::types::test_macros::serde::test_signed_serde!(Bound_isize); }
}