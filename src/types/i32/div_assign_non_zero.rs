use std::num::NonZeroI32;
use std::ops::DivAssign;

use crate::prelude::Bound_i32;

impl<const MIN: i32, const MAX: i32> DivAssign<NonZeroI32> for Bound_i32<MIN, MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn div_assign(&mut self, rhs: NonZeroI32) {
		self.set(i32::saturating_div(self.get(), rhs.get()));
	}
}

#[test]
fn test_div_assign() {
	use crate::types::test_macros::div_assign_non_zero::test_signed_div_assign_non_zero;

	test_signed_div_assign_non_zero!(Bound_i32, NonZeroI32);
}