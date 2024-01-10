use std::ops::MulAssign;
use crate::prelude::{Bound_usize, PadTo_i64, SqueezeTo_i64, SqueezeTo_usize};

impl<T, const MIN: usize, const MAX: usize> MulAssign<T> for Bound_usize<MIN, MAX> where T: SqueezeTo_i64 {
	#[inline(always)]
	fn mul_assign(&mut self, rhs: T) {
		self.set(i64::saturating_mul(self.get().pad_to_i64(), rhs.squeeze_to_i64()).squeeze_to_usize());
	}
}

#[cfg(test)]
pub mod tests {
	use crate::prelude::Bound_usize;
	use crate::types::test_macros::mul::{test_mul_unsigned_by_signed, test_mul_unsigned_by_unsigned};

	#[test] fn test_i8() { test_mul_unsigned_by_signed!(Bound_usize, i8); }
	#[test] fn test_i16() { test_mul_unsigned_by_signed!(Bound_usize, i16); }
	#[test] fn test_i32() { test_mul_unsigned_by_signed!(Bound_usize, i32); }
	#[test] fn test_i64() { test_mul_unsigned_by_signed!(Bound_usize, i64); }
	#[test] fn test_i128() { test_mul_unsigned_by_signed!(Bound_usize, i128); }
	#[test] fn test_isize() { test_mul_unsigned_by_signed!(Bound_usize, isize); }
	
	#[test] fn test_u8() { test_mul_unsigned_by_unsigned!(Bound_usize, u8); }
	#[test] fn test_u16() { test_mul_unsigned_by_unsigned!(Bound_usize, u16); }
	#[test] fn test_u32() { test_mul_unsigned_by_unsigned!(Bound_usize, u32); }
	#[test] fn test_u64() { test_mul_unsigned_by_unsigned!(Bound_usize, u64); }
	#[test] fn test_u128() { test_mul_unsigned_by_unsigned!(Bound_usize, u128); }
	#[test] fn test_usize() { test_mul_unsigned_by_unsigned!(Bound_usize, usize); }
}