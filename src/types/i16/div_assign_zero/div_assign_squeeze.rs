use std::ops::DivAssign;

use crate::prelude::{Bound_i16, PadTo_i32, SqueezeTo_i16, SqueezeTo_i32};

impl<T, const MIN: i16, const MAX: i16> DivAssign<T> for Bound_i16<MIN, MAX> where T: SqueezeTo_i32 {
	/// # Panics
	/// If rhs is 0.
	/// 
	/// Does not panic on overflowing, this is saturated.
	#[inline(always)]
	fn div_assign(&mut self, rhs: T) {
		self.set(i32::saturating_div(self.pad_to_i32(), rhs.squeeze_to_i32()).squeeze_to_i16());
	}
}

#[cfg(test)] 
pub mod tests {
	use crate::prelude::Bound_i16;
	use crate::prelude::test_macros::div::{test_div_signed_by_signed, test_div_signed_by_unsigned};

	#[test] fn test_i8() { test_div_signed_by_signed!(Bound_i16, i8); }
	#[test] fn test_i16() { test_div_signed_by_signed!(Bound_i16, i16); }
	#[test] fn test_i32() { test_div_signed_by_signed!(Bound_i16, i32); }
	#[test] fn test_i64() { test_div_signed_by_signed!(Bound_i16, i64); }
	#[test] fn test_i128() { test_div_signed_by_signed!(Bound_i16, i128); }
	#[test] fn test_isize() { test_div_signed_by_signed!(Bound_i16, isize); }

	#[test] fn test_u8() { test_div_signed_by_unsigned!(Bound_i16, u8); }
	#[test] fn test_u16() { test_div_signed_by_unsigned!(Bound_i16, u16); }
	#[test] fn test_u32() { test_div_signed_by_unsigned!(Bound_i16, u32); }
	#[test] fn test_u64() { test_div_signed_by_unsigned!(Bound_i16, u64); }
	#[test] fn test_u128() { test_div_signed_by_unsigned!(Bound_i16, u128); }
	#[test] fn test_usize() { test_div_signed_by_unsigned!(Bound_i16, usize); }
}

