use std::ops::DivAssign;

use crate::prelude::{PadTo_i64, SqueezeTo_i64, SqueezeTo_usize};
use crate::types::usize::*;

impl<T, const MIN: usize, const MAX: usize> DivAssign<T> for Bound_usize<MIN, MAX> where T: SqueezeTo_i64 {
	/// # Panics
	/// If rhs is 0.
	/// 
	/// Does not panic on overflowing, this is saturated.
	#[inline(always)]
	fn div_assign(&mut self, rhs: T) {
		self.set(i64::saturating_div(self.pad_to_i64(), rhs.squeeze_to_i64()).squeeze_to_usize());
	}
}

#[cfg(test)] 
pub mod tests {
	use crate::prelude::Bound_usize;
	use crate::prelude::test_macros::div::{test_div_unsigned_by_signed, test_div_unsigned_by_unsigned};

	#[test] fn test_i8() { test_div_unsigned_by_signed!(Bound_usize, i8); }
	#[test] fn test_i16() { test_div_unsigned_by_signed!(Bound_usize, i16); }
	#[test] fn test_i32() { test_div_unsigned_by_signed!(Bound_usize, i32); }
	#[test] fn test_i64() { test_div_unsigned_by_signed!(Bound_usize, i64); }
	#[test] fn test_i128() { test_div_unsigned_by_signed!(Bound_usize, i128); }
	#[test] fn test_isize() { test_div_unsigned_by_signed!(Bound_usize, isize); }

	#[test] fn test_u8() { test_div_unsigned_by_unsigned!(Bound_usize, u8); }
	#[test] fn test_u16() { test_div_unsigned_by_unsigned!(Bound_usize, u16); }
	#[test] fn test_u32() { test_div_unsigned_by_unsigned!(Bound_usize, u32); }
	#[test] fn test_u64() { test_div_unsigned_by_unsigned!(Bound_usize, u64); }
	#[test] fn test_u128() { test_div_unsigned_by_unsigned!(Bound_usize, u128); }
	#[test] fn test_usize() { test_div_unsigned_by_unsigned!(Bound_usize, usize); }
}

