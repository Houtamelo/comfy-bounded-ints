use std::num::NonZeroI128;
use std::ops::DivAssign;

use crate::prelude::Bound_i128;

impl<const MIN: i128, const MAX: i128> DivAssign<NonZeroI128> for Bound_i128<MIN, MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn div_assign(&mut self, rhs: NonZeroI128) {
		self.set(i128::saturating_div(self.get(), rhs.get()));
	}
}

#[test]
fn test_div_assign() {
	use crate::types::test_macros::div_assign_non_zero::test_signed_div_assign_non_zero;

	test_signed_div_assign_non_zero!(Bound_i128, NonZeroI128);
}