use std::ops::MulAssign;

use crate::prelude::{Bound_u128, PadTo_i128};

impl<T, const MIN: u128, const MAX: u128> MulAssign<T> for Bound_u128<MIN, MAX> where T: PadTo_i128 {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn mul_assign(&mut self, rhs: T) {
		let rhs = rhs.pad_to_i128();
		
		if rhs >= 0 {
			self.set(u128::saturating_mul(self.get(), rhs as u128));
		} else {
			self.set(Self::bound_lower());
		}
	}
}

impl<const MIN: u128, const MAX: u128> MulAssign<u128> for Bound_u128<MIN, MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn mul_assign(&mut self, rhs: u128) {
		self.set(u128::saturating_mul(self.get(), rhs));
	}
}

#[cfg(test)]
pub mod tests {
	use crate::prelude::Bound_u128;
	use crate::types::test_macros::mul::{test_mul_unsigned_by_signed, test_mul_unsigned_by_unsigned};

	#[test] fn test_i8() { test_mul_unsigned_by_signed!(Bound_u128, i8); }
	#[test] fn test_i16() { test_mul_unsigned_by_signed!(Bound_u128, i16); }
	#[test] fn test_i32() { test_mul_unsigned_by_signed!(Bound_u128, i32); }
	#[test] fn test_i64() { test_mul_unsigned_by_signed!(Bound_u128, i64); }
	#[test] fn test_i128() { test_mul_unsigned_by_signed!(Bound_u128, i128); }
	#[test] fn test_isize() { test_mul_unsigned_by_signed!(Bound_u128, isize); }
	
	#[test] fn test_u8() { test_mul_unsigned_by_unsigned!(Bound_u128, u8); }
	#[test] fn test_u16() { test_mul_unsigned_by_unsigned!(Bound_u128, u16); }
	#[test] fn test_u32() { test_mul_unsigned_by_unsigned!(Bound_u128, u32); }
	#[test] fn test_u64() { test_mul_unsigned_by_unsigned!(Bound_u128, u64); }
	#[test] fn test_u128() { test_mul_unsigned_by_unsigned!(Bound_u128, u128); }
	#[test] fn test_usize() { test_mul_unsigned_by_unsigned!(Bound_u128, usize); }
}