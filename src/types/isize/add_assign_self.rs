use std::ops::AddAssign;
use crate::prelude::Bound_isize;

impl<const A_MIN: isize, const A_MAX: isize, const B_MIN: isize, const B_MAX: isize> AddAssign<Bound_isize<A_MIN, A_MAX>> for Bound_isize<B_MIN, B_MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn add_assign(&mut self, rhs: Bound_isize<A_MIN, A_MAX>) {
		self.set(isize::saturating_add(self.get(), rhs.get()));
	}
}

#[test]
fn test_add_assign() {
	use crate::types::test_macros::add_assign_self::test_signed_add_assign_self;

	test_signed_add_assign_self!(Bound_isize);
}