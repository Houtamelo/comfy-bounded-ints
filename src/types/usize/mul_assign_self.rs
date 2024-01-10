use std::ops::MulAssign;
use crate::prelude::Bound_usize;

impl<const A_MIN: usize, const A_MAX: usize, const B_MIN: usize, const B_MAX: usize> MulAssign<Bound_usize<A_MIN, A_MAX>> for Bound_usize<B_MIN, B_MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn mul_assign(&mut self, rhs: Bound_usize<A_MIN, A_MAX>) {
		self.set(usize::saturating_mul(self.get(), rhs.get()));
	}
}

#[test]
fn test_mul_assign() {
	use crate::types::test_macros::mul_assign_self::test_mul_assign_self_unsigned;
	
	test_mul_assign_self_unsigned!(Bound_usize);
}