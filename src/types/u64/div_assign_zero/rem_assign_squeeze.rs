use std::ops::{Rem, RemAssign};

use crate::prelude::{Bound_u64, PadTo_i128, SqueezeTo_i128, SqueezeTo_u64};

impl<T, const MIN: u64, const MAX: u64> RemAssign<T> for Bound_u64<MIN, MAX> where T: SqueezeTo_i128 {
	/// # Panics
	/// If rhs is 0.
	/// 
	/// Does not panic on overflowing, this is saturated.
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn rem_assign(&mut self, rhs: T) {
		self.set(i128::rem(self.pad_to_i128(), rhs.squeeze_to_i128()).squeeze_to_u64());
	}
}

#[cfg(test)]
pub mod tests {
	use crate::prelude::Bound_u64;
	use crate::types::test_macros::rem::{test_rem_unsigned_by_signed, test_rem_unsigned_by_unsigned};

	#[test] fn test_i8() { test_rem_unsigned_by_signed!(Bound_u64, i8); }
	#[test] fn test_i16() { test_rem_unsigned_by_signed!(Bound_u64, i16); }
	#[test] fn test_i32() { test_rem_unsigned_by_signed!(Bound_u64, i32); }
	#[test] fn test_i64() { test_rem_unsigned_by_signed!(Bound_u64, i64); }
	#[test] fn test_i128() { test_rem_unsigned_by_signed!(Bound_u64, i128); }
	#[test] fn test_isize() { test_rem_unsigned_by_signed!(Bound_u64, isize); }

	#[test] fn test_u8() { test_rem_unsigned_by_unsigned!(Bound_u64, u8); }
	#[test] fn test_u16() { test_rem_unsigned_by_unsigned!(Bound_u64, u16); }
	#[test] fn test_u32() { test_rem_unsigned_by_unsigned!(Bound_u64, u32); }
	#[test] fn test_u64() { test_rem_unsigned_by_unsigned!(Bound_u64, u64); }
	#[test] fn test_u128() { test_rem_unsigned_by_unsigned!(Bound_u64, u128); }
	#[test] fn test_usize() { test_rem_unsigned_by_unsigned!(Bound_u64, usize); }
}

