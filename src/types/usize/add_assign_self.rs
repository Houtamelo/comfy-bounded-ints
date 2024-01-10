use std::ops::AddAssign;
use crate::prelude::Bound_usize;

impl<const A_MIN: usize, const A_MAX: usize, const B_MIN: usize, const B_MAX: usize> AddAssign<Bound_usize<A_MIN, A_MAX>> for Bound_usize<B_MIN, B_MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn add_assign(&mut self, rhs: Bound_usize<A_MIN, A_MAX>) {
		self.set(usize::saturating_add(self.get(), rhs.get()));
	}
}

#[test]
fn test_add_assign() {
	use crate::types::test_macros::add_assign_self::test_unsigned_add_assign_self;

	test_unsigned_add_assign_self!(Bound_usize);
}