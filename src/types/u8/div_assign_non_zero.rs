use std::num::NonZeroU8;
use std::ops::DivAssign;

use crate::prelude::Bound_u8;

impl<const MIN: u8, const MAX: u8> DivAssign<NonZeroU8> for Bound_u8<MIN, MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn div_assign(&mut self, rhs: NonZeroU8) {
		self.set(u8::saturating_div(self.get(), rhs.get()));
	}
}

#[test]
fn test_div_assign() {
	use crate::types::test_macros::div_assign_non_zero::test_unsigned_div_assign_non_zero;

	test_unsigned_div_assign_non_zero!(Bound_u8, NonZeroU8);
}