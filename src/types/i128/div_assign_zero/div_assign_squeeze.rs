use std::ops::DivAssign;

use crate::prelude::SqueezeTo_i128;
use crate::types::i128::*;

impl<T, const MIN: i128, const MAX: i128> DivAssign<T> for Bound_i128<MIN, MAX> where T: SqueezeTo_i128 {
	/// # Panics
	/// If rhs is 0.
	/// 
	/// Does not panic on overflowing, this is saturated.
	#[inline(always)]
	fn div_assign(&mut self, rhs: T) {
		self.set(i128::saturating_div(self.get(), rhs.squeeze_to_i128()));
	}
}

#[cfg(test)] 
pub mod tests {
	use crate::prelude::Bound_i128;
	use crate::prelude::test_macros::div::{test_div_signed_by_signed, test_div_signed_by_unsigned};

	#[test] fn test_i8() { test_div_signed_by_signed!(Bound_i128, i8); }
	#[test] fn test_i16() { test_div_signed_by_signed!(Bound_i128, i16); }
	#[test] fn test_i32() { test_div_signed_by_signed!(Bound_i128, i32); }
	#[test] fn test_i64() { test_div_signed_by_signed!(Bound_i128, i64); }
	#[test] fn test_i128() { test_div_signed_by_signed!(Bound_i128, i128); }
	#[test] fn test_isize() { test_div_signed_by_signed!(Bound_i128, isize); }

	#[test] fn test_u8() { test_div_signed_by_unsigned!(Bound_i128, u8); }
	#[test] fn test_u16() { test_div_signed_by_unsigned!(Bound_i128, u16); }
	#[test] fn test_u32() { test_div_signed_by_unsigned!(Bound_i128, u32); }
	#[test] fn test_u64() { test_div_signed_by_unsigned!(Bound_i128, u64); }
	#[test] fn test_u128() { test_div_signed_by_unsigned!(Bound_i128, u128); }
	#[test] fn test_usize() { test_div_signed_by_unsigned!(Bound_i128, usize); }
}

