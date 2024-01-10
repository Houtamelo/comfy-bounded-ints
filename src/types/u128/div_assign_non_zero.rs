use std::num::NonZeroU128;
use std::ops::DivAssign;

use crate::prelude::Bound_u128;

impl<const MIN: u128, const MAX: u128> DivAssign<NonZeroU128> for Bound_u128<MIN, MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn div_assign(&mut self, rhs: NonZeroU128) {
		self.set(u128::saturating_div(self.get(), rhs.get()));
	}
}

#[test]
fn test_div_assign() {
	use crate::types::test_macros::div_assign_non_zero::test_unsigned_div_assign_non_zero;

	test_unsigned_div_assign_non_zero!(Bound_u128, NonZeroU128);
}