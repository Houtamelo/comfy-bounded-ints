use std::ops::DivAssign;

use crate::prelude::Bound_u128;

impl<const A_MIN: u128, const A_MAX: u128, const B_MIN: u128, const B_MAX: u128> DivAssign<Bound_u128<A_MIN, A_MAX>> for Bound_u128<B_MIN, B_MAX> {
	/// # Panics
	/// If rhs is 0.
	/// 
	/// Does not panic on overflowing, this is saturated.
	#[inline(always)]
	fn div_assign(&mut self, rhs: Bound_u128<A_MIN, A_MAX>) {
		self.set(u128::saturating_div(self.get(), rhs.get()));
	}
}

#[test]
fn test_div_assign() {
	let mut bound = Bound_u128::<0, 100>::new(100);
	bound /= Bound_u128::<5, 20>::new(2);
	assert_eq!(bound.get(), 20);
	bound /= Bound_u128::<6, 20>::new(0);
	assert_eq!(bound.get(), 3);
	bound /= Bound_u128::<20, 50>::new(100);
	assert_eq!(bound.get(), 0);
}

#[test]
#[should_panic]
fn test_div_assign_panic() {
	let mut bound = Bound_u128::<0, 100>::new(50);
	bound /= Bound_u128::<0, 50>::new(0);
}
