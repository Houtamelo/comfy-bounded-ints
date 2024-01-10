use std::num::NonZeroU64;
use std::ops::DivAssign;

use crate::prelude::Bound_u64;

impl<const MIN: u64, const MAX: u64> DivAssign<NonZeroU64> for Bound_u64<MIN, MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn div_assign(&mut self, rhs: NonZeroU64) {
		self.set(u64::saturating_div(self.get(), rhs.get()));
	}
}

#[test]
fn test_div_assign() {
	use crate::types::test_macros::div_assign_non_zero::test_unsigned_div_assign_non_zero;

	test_unsigned_div_assign_non_zero!(Bound_u64, NonZeroU64);
}