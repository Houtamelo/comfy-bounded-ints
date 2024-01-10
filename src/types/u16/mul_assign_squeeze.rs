use std::ops::MulAssign;

use crate::prelude::{Bound_u16, PadTo_i32, SqueezeTo_i32, SqueezeTo_u16};

impl<T, const MIN: u16, const MAX: u16> MulAssign<T> for Bound_u16<MIN, MAX> where T: SqueezeTo_i32 {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn mul_assign(&mut self, rhs: T) {
		self.set(i32::saturating_mul(self.get().pad_to_i32(), rhs.squeeze_to_i32()).squeeze_to_u16());
	}
}

#[cfg(test)]
pub mod tests {
	use crate::prelude::Bound_u16;
	use crate::types::test_macros::mul::{test_mul_unsigned_by_signed, test_mul_unsigned_by_unsigned};

	#[test] fn test_i8() { test_mul_unsigned_by_signed!(Bound_u16, i8); }
	#[test] fn test_i16() { test_mul_unsigned_by_signed!(Bound_u16, i16); }
	#[test] fn test_i32() { test_mul_unsigned_by_signed!(Bound_u16, i32); }
	#[test] fn test_i64() { test_mul_unsigned_by_signed!(Bound_u16, i64); }
	#[test] fn test_i128() { test_mul_unsigned_by_signed!(Bound_u16, i128); }
	#[test] fn test_isize() { test_mul_unsigned_by_signed!(Bound_u16, isize); }
	
	#[test] fn test_u8() { test_mul_unsigned_by_unsigned!(Bound_u16, u8); }
	#[test] fn test_u16() { test_mul_unsigned_by_unsigned!(Bound_u16, u16); }
	#[test] fn test_u32() { test_mul_unsigned_by_unsigned!(Bound_u16, u32); }
	#[test] fn test_u64() { test_mul_unsigned_by_unsigned!(Bound_u16, u64); }
	#[test] fn test_u128() { test_mul_unsigned_by_unsigned!(Bound_u16, u128); }
	#[test] fn test_usize() { test_mul_unsigned_by_unsigned!(Bound_u16, usize); }
}