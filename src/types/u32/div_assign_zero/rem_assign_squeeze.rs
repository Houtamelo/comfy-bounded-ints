use std::ops::{Rem, RemAssign};

use crate::prelude::{Bound_u32, PadTo_i64, SqueezeTo_i64, SqueezeTo_u32};

impl<T, const MIN: u32, const MAX: u32> RemAssign<T> for Bound_u32<MIN, MAX> where T: SqueezeTo_i64 {
	/// # Panics
	/// If rhs is 0.
	/// 
	/// Does not panic on overflowing, this is saturated.
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn rem_assign(&mut self, rhs: T) {
		self.set(i64::rem(self.pad_to_i64(), rhs.squeeze_to_i64()).squeeze_to_u32());
	}
}

#[cfg(test)]
pub mod tests {
	use crate::prelude::Bound_u32;
	use crate::types::test_macros::rem::{test_rem_unsigned_by_signed, test_rem_unsigned_by_unsigned};

	#[test] fn test_i8() { test_rem_unsigned_by_signed!(Bound_u32, i8); }
	#[test] fn test_i16() { test_rem_unsigned_by_signed!(Bound_u32, i16); }
	#[test] fn test_i32() { test_rem_unsigned_by_signed!(Bound_u32, i32); }
	#[test] fn test_i64() { test_rem_unsigned_by_signed!(Bound_u32, i64); }
	#[test] fn test_i128() { test_rem_unsigned_by_signed!(Bound_u32, i128); }
	#[test] fn test_isize() { test_rem_unsigned_by_signed!(Bound_u32, isize); }

	#[test] fn test_u8() { test_rem_unsigned_by_unsigned!(Bound_u32, u8); }
	#[test] fn test_u16() { test_rem_unsigned_by_unsigned!(Bound_u32, u16); }
	#[test] fn test_u32() { test_rem_unsigned_by_unsigned!(Bound_u32, u32); }
	#[test] fn test_u64() { test_rem_unsigned_by_unsigned!(Bound_u32, u64); }
	#[test] fn test_u128() { test_rem_unsigned_by_unsigned!(Bound_u32, u128); }
	#[test] fn test_usize() { test_rem_unsigned_by_unsigned!(Bound_u32, usize); }
}

