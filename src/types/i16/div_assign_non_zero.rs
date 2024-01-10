use std::num::NonZeroI16;
use std::ops::DivAssign;

use crate::prelude::Bound_i16;

impl<const MIN: i16, const MAX: i16> DivAssign<NonZeroI16> for Bound_i16<MIN, MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn div_assign(&mut self, rhs: NonZeroI16) {
		self.set(i16::saturating_div(self.get(), rhs.get()));
	}
}

#[test]
fn test_div_assign() {
	use crate::types::test_macros::div_assign_non_zero::test_signed_div_assign_non_zero;

	test_signed_div_assign_non_zero!(Bound_i16, NonZeroI16);
}