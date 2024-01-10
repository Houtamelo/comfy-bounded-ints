use std::ops::DivAssign;

use crate::prelude::{PadTo_i128, SqueezeTo_i128, SqueezeTo_i64};
use crate::types::i64::*;

impl<T, const MIN: i64, const MAX: i64> DivAssign<T> for Bound_i64<MIN, MAX> where T: SqueezeTo_i128 {
	/// # Panics
	/// If rhs is 0.
	/// 
	/// Does not panic on overflowing, this is saturated.
	#[inline(always)]
	fn div_assign(&mut self, rhs: T) {
		self.set(i128::saturating_div(self.pad_to_i128(), rhs.squeeze_to_i128()).squeeze_to_i64());
	}
}

#[cfg(test)] 
pub mod tests {
	use crate::prelude::Bound_i64;
	use crate::prelude::test_macros::div::{test_div_signed_by_signed, test_div_signed_by_unsigned};

	#[test] fn test_i8() { test_div_signed_by_signed!(Bound_i64, i8); }
	#[test] fn test_i16() { test_div_signed_by_signed!(Bound_i64, i16); }
	#[test] fn test_i32() { test_div_signed_by_signed!(Bound_i64, i32); }
	#[test] fn test_i64() { test_div_signed_by_signed!(Bound_i64, i64); }
	#[test] fn test_i128() { test_div_signed_by_signed!(Bound_i64, i128); }
	#[test] fn test_isize() { test_div_signed_by_signed!(Bound_i64, isize); }

	#[test] fn test_u8() { test_div_signed_by_unsigned!(Bound_i64, u8); }
	#[test] fn test_u16() { test_div_signed_by_unsigned!(Bound_i64, u16); }
	#[test] fn test_u32() { test_div_signed_by_unsigned!(Bound_i64, u32); }
	#[test] fn test_u64() { test_div_signed_by_unsigned!(Bound_i64, u64); }
	#[test] fn test_u128() { test_div_signed_by_unsigned!(Bound_i64, u128); }
	#[test] fn test_usize() { test_div_signed_by_unsigned!(Bound_i64, usize); }
}

