use std::ops::SubAssign;
use crate::prelude::{Bound_isize, PadTo_i64, SqueezeTo_i64, SqueezeTo_isize};

impl<T, const MIN: isize, const MAX: isize> SubAssign<T> for Bound_isize<MIN, MAX> where T: SqueezeTo_i64 {
	#[inline(always)]
	fn sub_assign(&mut self, rhs: T) {
		self.set(i64::saturating_sub(self.get().pad_to_i64(), rhs.squeeze_to_i64()).squeeze_to_isize());
	}
}

#[cfg(test)]
pub mod tests {
	use crate::prelude::Bound_isize;
	use crate::types::test_macros::sub::{test_sub_signed_by_signed, test_sub_signed_by_unsigned};

	#[test] fn test_i8() { test_sub_signed_by_signed!(Bound_isize, i8); }
	#[test] fn test_i16() { test_sub_signed_by_signed!(Bound_isize, i16); }
	#[test] fn test_i32() { test_sub_signed_by_signed!(Bound_isize, i32); }
	#[test] fn test_i64() { test_sub_signed_by_signed!(Bound_isize, i64); }
	#[test] fn test_i128() { test_sub_signed_by_signed!(Bound_isize, i128); }
	#[test] fn test_isize() { test_sub_signed_by_signed!(Bound_isize, isize); }
	
	#[test] fn test_u8() { test_sub_signed_by_unsigned!(Bound_isize, u8); }
	#[test] fn test_u16() { test_sub_signed_by_unsigned!(Bound_isize, u16); }
	#[test] fn test_u32() { test_sub_signed_by_unsigned!(Bound_isize, u32); }
	#[test] fn test_u64() { test_sub_signed_by_unsigned!(Bound_isize, u64); }
	#[test] fn test_u128() { test_sub_signed_by_unsigned!(Bound_isize, u128); }
	#[test] fn test_usize() { test_sub_signed_by_unsigned!(Bound_isize, usize); }
}