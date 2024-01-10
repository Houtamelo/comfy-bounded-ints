use std::ops::MulAssign;

use crate::prelude::{Bound_i128, SqueezeTo_i128};

impl<T, const MIN: i128, const MAX: i128> MulAssign<T> for Bound_i128<MIN, MAX> where T: SqueezeTo_i128 {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn mul_assign(&mut self, rhs: T) {
		self.set(i128::saturating_mul(self.get(), rhs.squeeze_to_i128()));
	}
}

#[cfg(test)]
pub mod tests {
	use crate::prelude::Bound_i128;
	use crate::types::test_macros::mul::{test_mul_signed_by_signed, test_mul_signed_by_unsigned};

	#[test] fn test_i8() { test_mul_signed_by_signed!(Bound_i128, i8); }
	#[test] fn test_i16() { test_mul_signed_by_signed!(Bound_i128, i16); }
	#[test] fn test_i32() { test_mul_signed_by_signed!(Bound_i128, i32); }
	#[test] fn test_i64() { test_mul_signed_by_signed!(Bound_i128, i64); }
	#[test] fn test_i128() { test_mul_signed_by_signed!(Bound_i128, i128); }
	#[test] fn test_isize() { test_mul_signed_by_signed!(Bound_i128, isize); }
	
	#[test] fn test_u8() { test_mul_signed_by_unsigned!(Bound_i128, u8); }
	#[test] fn test_u16() { test_mul_signed_by_unsigned!(Bound_i128, u16); }
	#[test] fn test_u32() { test_mul_signed_by_unsigned!(Bound_i128, u32); }
	#[test] fn test_u64() { test_mul_signed_by_unsigned!(Bound_i128, u64); }
	#[test] fn test_u128() { test_mul_signed_by_unsigned!(Bound_i128, u128); }
	#[test] fn test_usize() { test_mul_signed_by_unsigned!(Bound_i128, usize); }
}