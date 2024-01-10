use std::ops::MulAssign;

use crate::prelude::{Bound_i32, PadTo_i64, SqueezeTo_i32, SqueezeTo_i64};

impl<T, const MIN: i32, const MAX: i32> MulAssign<T> for Bound_i32<MIN, MAX> where T: SqueezeTo_i64 {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn mul_assign(&mut self, rhs: T) {
		self.set(i64::saturating_mul(self.get().pad_to_i64(), rhs.squeeze_to_i64()).squeeze_to_i32());
	}
}

#[cfg(test)]
pub mod tests {
	use crate::prelude::Bound_i32;
	use crate::types::test_macros::mul::{test_mul_signed_by_signed, test_mul_signed_by_unsigned};

	#[test] fn test_i8() { test_mul_signed_by_signed!(Bound_i32, i8); }
	#[test] fn test_i16() { test_mul_signed_by_signed!(Bound_i32, i16); }
	#[test] fn test_i32() { test_mul_signed_by_signed!(Bound_i32, i32); }
	#[test] fn test_i64() { test_mul_signed_by_signed!(Bound_i32, i64); }
	#[test] fn test_i128() { test_mul_signed_by_signed!(Bound_i32, i128); }
	#[test] fn test_isize() { test_mul_signed_by_signed!(Bound_i32, isize); }
	
	#[test] fn test_u8() { test_mul_signed_by_unsigned!(Bound_i32, u8); }
	#[test] fn test_u16() { test_mul_signed_by_unsigned!(Bound_i32, u16); }
	#[test] fn test_u32() { test_mul_signed_by_unsigned!(Bound_i32, u32); }
	#[test] fn test_u64() { test_mul_signed_by_unsigned!(Bound_i32, u64); }
	#[test] fn test_u128() { test_mul_signed_by_unsigned!(Bound_i32, u128); }
	#[test] fn test_usize() { test_mul_signed_by_unsigned!(Bound_i32, usize); }
}
