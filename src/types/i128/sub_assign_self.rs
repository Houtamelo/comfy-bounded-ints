use std::ops::SubAssign;

use crate::prelude::Bound_i128;

impl<const A_MIN: i128, const A_MAX: i128, const B_MIN: i128, const B_MAX: i128> SubAssign<Bound_i128<A_MIN, A_MAX>> for Bound_i128<B_MIN, B_MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn sub_assign(&mut self, rhs: Bound_i128<A_MIN, A_MAX>) {
		self.set(i128::saturating_sub(self.get(), rhs.get()));
	}
}

#[test]
fn test_sub_assign() {
	use crate::types::test_macros::sub_assign_self::test_sub_assign_self_signed;
	
	test_sub_assign_self_signed!(Bound_i128);
}