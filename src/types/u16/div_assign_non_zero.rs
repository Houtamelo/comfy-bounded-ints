use std::num::NonZeroU16;
use std::ops::DivAssign;

use crate::prelude::Bound_u16;

impl<const MIN: u16, const MAX: u16> DivAssign<NonZeroU16> for Bound_u16<MIN, MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn div_assign(&mut self, rhs: NonZeroU16) {
		self.set(u16::saturating_div(self.get(), rhs.get()));
	}
}

#[test]
fn test_div_assign() {
	use crate::types::test_macros::div_assign_non_zero::test_unsigned_div_assign_non_zero;

	test_unsigned_div_assign_non_zero!(Bound_u16, NonZeroU16);
}