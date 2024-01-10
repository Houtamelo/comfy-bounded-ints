use std::ops::MulAssign;
use crate::prelude::Bound_u128;

impl<const A_MIN: u128, const A_MAX: u128, const B_MIN: u128, const B_MAX: u128> MulAssign<Bound_u128<A_MIN, A_MAX>> for Bound_u128<B_MIN, B_MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn mul_assign(&mut self, rhs: Bound_u128<A_MIN, A_MAX>) {
		self.set(u128::saturating_mul(self.get(), rhs.get()));
	}
}

#[test]
fn test_mul_assign() {
	use crate::types::test_macros::mul_assign_self::test_mul_assign_self_unsigned;
	
	test_mul_assign_self_unsigned!(Bound_u128);
}