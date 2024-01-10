use std::ops::AddAssign;
use crate::prelude::Bound_i16;

impl<const A_MIN: i16, const A_MAX: i16, const B_MIN: i16, const B_MAX: i16> AddAssign<Bound_i16<A_MIN, A_MAX>> for Bound_i16<B_MIN, B_MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn add_assign(&mut self, rhs: Bound_i16<A_MIN, A_MAX>) {
		self.set(i16::saturating_add(self.get(), rhs.get()));
	}
}

#[test]
fn test_add_assign() {
	use crate::types::test_macros::add_assign_self::test_signed_add_assign_self;

	test_signed_add_assign_self!(Bound_i16);
}