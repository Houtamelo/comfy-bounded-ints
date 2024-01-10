use std::ops::AddAssign;
use crate::prelude::Bound_u128;

impl<const A_MIN: u128, const A_MAX: u128, const B_MIN: u128, const B_MAX: u128> AddAssign<Bound_u128<A_MIN, A_MAX>> for Bound_u128<B_MIN, B_MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn add_assign(&mut self, rhs: Bound_u128<A_MIN, A_MAX>) {
		self.set(u128::saturating_add(self.get(), rhs.get()));
	}
}

#[test]
fn test_add_assign() {
	use crate::types::test_macros::add_assign_self::test_unsigned_add_assign_self;

	test_unsigned_add_assign_self!(Bound_u128);
}