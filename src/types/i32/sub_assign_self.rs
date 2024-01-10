use std::ops::SubAssign;

use crate::prelude::Bound_i32;

impl<const A_MIN: i32, const A_MAX: i32, const B_MIN: i32, const B_MAX: i32> SubAssign<Bound_i32<A_MIN, A_MAX>> for Bound_i32<B_MIN, B_MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn sub_assign(&mut self, rhs: Bound_i32<A_MIN, A_MAX>) {
		self.set(i32::saturating_sub(self.get(), rhs.get()));
	}
}

#[test]
fn test_sub_assign() {
	use crate::types::test_macros::sub_assign_self::test_sub_assign_self_signed;
	
	test_sub_assign_self_signed!(Bound_i32);
}