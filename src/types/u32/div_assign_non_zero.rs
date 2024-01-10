use std::num::NonZeroU32;
use std::ops::DivAssign;

use crate::prelude::Bound_u32;

impl<const MIN: u32, const MAX: u32> DivAssign<NonZeroU32> for Bound_u32<MIN, MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn div_assign(&mut self, rhs: NonZeroU32) {
		self.set(u32::saturating_div(self.get(), rhs.get()));
	}
}

#[test]
fn test_div_assign() {
	use crate::types::test_macros::div_assign_non_zero::test_unsigned_div_assign_non_zero;

	test_unsigned_div_assign_non_zero!(Bound_u32, NonZeroU32);
}