use std::ops::DivAssign;

use crate::prelude::Bound_u16;

impl<const A_MIN: u16, const A_MAX: u16, const B_MIN: u16, const B_MAX: u16> DivAssign<Bound_u16<A_MIN, A_MAX>> for Bound_u16<B_MIN, B_MAX> {
	/// # Panics
	/// If rhs is 0.
	/// 
	/// Does not panic on overflowing, this is saturated.
	#[inline(always)]
	fn div_assign(&mut self, rhs: Bound_u16<A_MIN, A_MAX>) {
		self.set(u16::saturating_div(self.get(), rhs.get()));
	}
}

#[test]
fn test_div_assign() {
	let mut bound = Bound_u16::<0, 100>::new(100);
	bound /= Bound_u16::<5, 20>::new(2);
	assert_eq!(bound.get(), 20);
	bound /= Bound_u16::<6, 20>::new(0);
	assert_eq!(bound.get(), 3);
	bound /= Bound_u16::<20, 50>::new(100);
	assert_eq!(bound.get(), 0);
}

#[test]
#[should_panic]
fn test_div_assign_panic() {
	let mut bound = Bound_u16::<0, 100>::new(50);
	bound /= Bound_u16::<0, 50>::new(0);
}
