use std::ops::SubAssign;
use crate::prelude::{Bound_u8, PadTo_i16, SqueezeTo_i16, SqueezeTo_u8};

impl<T, const MIN: u8, const MAX: u8> SubAssign<T> for Bound_u8<MIN, MAX> where T: SqueezeTo_i16 {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn sub_assign(&mut self, rhs: T) {
		self.set(i16::saturating_sub(self.get().pad_to_i16(), rhs.squeeze_to_i16()).squeeze_to_u8());
	}
}

#[cfg(test)]
pub mod tests {
	use crate::prelude::Bound_u8;
	use crate::types::test_macros::sub::{test_sub_unsigned_by_signed, test_sub_unsigned_by_unsigned};

	#[test] fn test_i8() { test_sub_unsigned_by_signed!(Bound_u8, i8); }
	#[test] fn test_i16() { test_sub_unsigned_by_signed!(Bound_u8, i16); }
	#[test] fn test_i32() { test_sub_unsigned_by_signed!(Bound_u8, i32); }
	#[test] fn test_i64() { test_sub_unsigned_by_signed!(Bound_u8, i64); }
	#[test] fn test_i128() { test_sub_unsigned_by_signed!(Bound_u8, i128); }
	#[test] fn test_isize() { test_sub_unsigned_by_signed!(Bound_u8, isize); }
	
	#[test] fn test_u8() { test_sub_unsigned_by_unsigned!(Bound_u8, u8); }
	#[test] fn test_u16() { test_sub_unsigned_by_unsigned!(Bound_u8, u16); }
	#[test] fn test_u32() { test_sub_unsigned_by_unsigned!(Bound_u8, u32); }
	#[test] fn test_u64() { test_sub_unsigned_by_unsigned!(Bound_u8, u64); }
	#[test] fn test_u128() { test_sub_unsigned_by_unsigned!(Bound_u8, u128); }
	#[test] fn test_usize() { test_sub_unsigned_by_unsigned!(Bound_u8, usize); }
}