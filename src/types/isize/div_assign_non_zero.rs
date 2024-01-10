use std::num::NonZeroIsize;
use std::ops::DivAssign;

use crate::prelude::Bound_isize;

impl<const MIN: isize, const MAX: isize> DivAssign<NonZeroIsize> for Bound_isize<MIN, MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn div_assign(&mut self, rhs: NonZeroIsize) {
		self.set(isize::saturating_div(self.get(), rhs.get()));
	}
}

#[test]
fn test_div_assign() {
	use crate::types::test_macros::div_assign_non_zero::test_signed_div_assign_non_zero;

	test_signed_div_assign_non_zero!(Bound_isize, NonZeroIsize);
}