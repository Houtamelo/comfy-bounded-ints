use std::ops::SubAssign;

use crate::prelude::Bound_i64;

impl<const A_MIN: i64, const A_MAX: i64, const B_MIN: i64, const B_MAX: i64> SubAssign<Bound_i64<A_MIN, A_MAX>> for Bound_i64<B_MIN, B_MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn sub_assign(&mut self, rhs: Bound_i64<A_MIN, A_MAX>) {
		self.set(i64::saturating_sub(self.get(), rhs.get()));
	}
}

#[test]
fn test_sub_assign() {
	use crate::types::test_macros::sub_assign_self::test_sub_assign_self_signed;
	
	test_sub_assign_self_signed!(Bound_i64);
}