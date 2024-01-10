use std::ops::MulAssign;
use crate::prelude::Bound_isize;

impl<const A_MIN: isize, const A_MAX: isize, const B_MIN: isize, const B_MAX: isize> MulAssign<Bound_isize<A_MIN, A_MAX>> for Bound_isize<B_MIN, B_MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn mul_assign(&mut self, rhs: Bound_isize<A_MIN, A_MAX>) {
		self.set(isize::saturating_mul(self.get(), rhs.get()));
	}
}

#[test]
fn test_mul_assign() {
	use crate::types::test_macros::mul_assign_self::test_mul_assign_self_signed;
	
	test_mul_assign_self_signed!(Bound_isize);
}