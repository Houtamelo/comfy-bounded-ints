use std::num::NonZeroI64;
use std::ops::DivAssign;

use crate::prelude::Bound_i64;

impl<const MIN: i64, const MAX: i64> DivAssign<NonZeroI64> for Bound_i64<MIN, MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn div_assign(&mut self, rhs: NonZeroI64) {
		self.set(i64::saturating_div(self.get(), rhs.get()));
	}
}

#[test]
fn test_div_assign() {
	use crate::types::test_macros::div_assign_non_zero::test_signed_div_assign_non_zero;

	test_signed_div_assign_non_zero!(Bound_i64, NonZeroI64);
}