use std::ops::AddAssign;
use crate::prelude::{Bound_isize, PadTo_i128, SqueezeTo_i128, SqueezeTo_isize};

impl<T, const MIN: isize, const MAX: isize> AddAssign<T> for Bound_isize<MIN, MAX> where T: SqueezeTo_i128 {
	#[inline(always)]
	fn add_assign(&mut self, rhs: T) {
		self.set(i128::saturating_add(self.get().pad_to_i128(), rhs.squeeze_to_i128()).squeeze_to_isize());
	}
}

#[cfg(test)] 
pub mod tests {
	use crate::prelude::Bound_isize;
	use crate::types::test_macros::add::{test_add_signed_by_signed, test_add_signed_by_unsigned};

	#[test] fn test_i8() { test_add_signed_by_signed!(Bound_isize, i8); }
	#[test] fn test_i16() { test_add_signed_by_signed!(Bound_isize, i16); }
	#[test] fn test_i32() { test_add_signed_by_signed!(Bound_isize, i32); }
	#[test] fn test_i64() { test_add_signed_by_signed!(Bound_isize, i64); }
	#[test] fn test_i128() { test_add_signed_by_signed!(Bound_isize, i128); }
	#[test] fn test_isize() { test_add_signed_by_signed!(Bound_isize, isize); }
	
	#[test] fn test_u8() { test_add_signed_by_unsigned!(Bound_isize, u8); }
	#[test] fn test_u16() { test_add_signed_by_unsigned!(Bound_isize, u16); }
	#[test] fn test_u32() { test_add_signed_by_unsigned!(Bound_isize, u32); }
	#[test] fn test_u64() { test_add_signed_by_unsigned!(Bound_isize, u64); }
	#[test] fn test_u128() { test_add_signed_by_unsigned!(Bound_isize, u128); }
	#[test] fn test_usize() { test_add_signed_by_unsigned!(Bound_isize, usize); }
}