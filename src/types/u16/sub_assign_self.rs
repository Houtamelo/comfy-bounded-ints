use std::ops::SubAssign;
use crate::prelude::Bound_u16;

impl<const A_MIN: u16, const A_MAX: u16, const B_MIN: u16, const B_MAX: u16> SubAssign<Bound_u16<A_MIN, A_MAX>> for Bound_u16<B_MIN, B_MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn sub_assign(&mut self, rhs: Bound_u16<A_MIN, A_MAX>) {
		self.set(u16::saturating_sub(self.get(), rhs.get()));
	}
}

#[test]
fn test_sub_assign() {
	use crate::types::test_macros::sub_assign_self::test_sub_assign_self_unsigned;
	
	test_sub_assign_self_unsigned!(Bound_u16);
}