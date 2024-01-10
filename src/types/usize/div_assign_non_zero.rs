use std::num::NonZeroUsize;
use std::ops::DivAssign;

use crate::prelude::Bound_usize;

impl<const MIN: usize, const MAX: usize> DivAssign<NonZeroUsize> for Bound_usize<MIN, MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn div_assign(&mut self, rhs: NonZeroUsize) {
		self.set(usize::saturating_div(self.get(), rhs.get()));
	}
}

#[test]
fn test_div_assign() {
	use crate::types::test_macros::div_assign_non_zero::test_unsigned_div_assign_non_zero;

	test_unsigned_div_assign_non_zero!(Bound_usize, NonZeroUsize);
}