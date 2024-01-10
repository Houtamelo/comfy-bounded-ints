use std::ops::SubAssign;
use crate::prelude::{Bound_usize, PadTo_i128, SqueezeTo_i128, SqueezeTo_usize};

impl<T, const MIN: usize, const MAX: usize> SubAssign<T> for Bound_usize<MIN, MAX> where T: SqueezeTo_i128 {
	#[inline(always)]
	fn sub_assign(&mut self, rhs: T) {
		self.set(i128::saturating_sub(self.get().pad_to_i128(), rhs.squeeze_to_i128()).squeeze_to_usize());
	}
}

#[cfg(test)]
pub mod tests {
	use crate::prelude::Bound_usize;
	use crate::types::test_macros::sub::{test_sub_unsigned_by_signed, test_sub_unsigned_by_unsigned};

	#[test] fn test_i8() { test_sub_unsigned_by_signed!(Bound_usize, i8); }
	#[test] fn test_i16() { test_sub_unsigned_by_signed!(Bound_usize, i16); }
	#[test] fn test_i32() { test_sub_unsigned_by_signed!(Bound_usize, i32); }
	#[test] fn test_i64() { test_sub_unsigned_by_signed!(Bound_usize, i64); }
	#[test] fn test_i128() { test_sub_unsigned_by_signed!(Bound_usize, i128); }
	#[test] fn test_isize() { test_sub_unsigned_by_signed!(Bound_usize, isize); }
	
	#[test] fn test_u8() { test_sub_unsigned_by_unsigned!(Bound_usize, u8); }
	#[test] fn test_u16() { test_sub_unsigned_by_unsigned!(Bound_usize, u16); }
	#[test] fn test_u32() { test_sub_unsigned_by_unsigned!(Bound_usize, u32); }
	#[test] fn test_u64() { test_sub_unsigned_by_unsigned!(Bound_usize, u64); }
	#[test] fn test_u128() { test_sub_unsigned_by_unsigned!(Bound_usize, u128); }
	#[test] fn test_usize() { test_sub_unsigned_by_unsigned!(Bound_usize, usize); }
}