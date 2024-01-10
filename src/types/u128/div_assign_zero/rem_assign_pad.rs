use std::ops::{Rem, RemAssign};

use crate::prelude::{Bound_u128, PadTo_i128};

impl<T, const MIN: u128, const MAX: u128> RemAssign<T> for Bound_u128<MIN, MAX> where T: PadTo_i128 {
	/// # Panics
	/// If rhs is 0.
	/// 
	/// Does not panic on overflowing, this is saturated.
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn rem_assign(&mut self, rhs: T) {
		self.set(u128::rem(self.get(), rhs.pad_to_i128().saturating_abs() as u128)); // regardless of sign, rhs is positive, so dividing by positive or negative is the same
	}
}

impl<const MIN: u128, const MAX: u128> RemAssign<u128> for Bound_u128<MIN, MAX> {
	/// # Panics
	/// If rhs is 0.
	/// 
	/// Does not panic on overflowing, this is saturated. (This cannot overflow anyway)
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn rem_assign(&mut self, rhs: u128) {
		self.set(u128::rem(self.get(), rhs));
	}
}

#[cfg(test)]
pub mod tests {
	use crate::prelude::Bound_u128;
	use crate::types::test_macros::rem::{test_rem_unsigned_by_signed, test_rem_unsigned_by_unsigned};

	#[test] fn test_i8() { test_rem_unsigned_by_signed!(Bound_u128, i8); }
	#[test] fn test_i16() { test_rem_unsigned_by_signed!(Bound_u128, i16); }
	#[test] fn test_i32() { test_rem_unsigned_by_signed!(Bound_u128, i32); }
	#[test] fn test_i64() { test_rem_unsigned_by_signed!(Bound_u128, i64); }
	#[test] fn test_i128() { test_rem_unsigned_by_signed!(Bound_u128, i128); }
	#[test] fn test_isize() { test_rem_unsigned_by_signed!(Bound_u128, isize); }

	#[test] fn test_u8() { test_rem_unsigned_by_unsigned!(Bound_u128, u8); }
	#[test] fn test_u16() { test_rem_unsigned_by_unsigned!(Bound_u128, u16); }
	#[test] fn test_u32() { test_rem_unsigned_by_unsigned!(Bound_u128, u32); }
	#[test] fn test_u64() { test_rem_unsigned_by_unsigned!(Bound_u128, u64); }
	#[test] fn test_u128() { test_rem_unsigned_by_unsigned!(Bound_u128, u128); }
	#[test] fn test_usize() { test_rem_unsigned_by_unsigned!(Bound_u128, usize); }
}

