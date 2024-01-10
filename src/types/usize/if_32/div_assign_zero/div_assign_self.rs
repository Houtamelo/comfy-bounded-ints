use std::ops::DivAssign;

use crate::prelude::Bound_usize;

impl<const A_MIN: usize, const A_MAX: usize, const B_MIN: usize, const B_MAX: usize> DivAssign<Bound_usize<A_MIN, A_MAX>> for Bound_usize<B_MIN, B_MAX> {
	/// # Panics
	/// If rhs is 0.
	/// 
	/// Does not panic on overflowing, this is saturated.
	#[inline(always)]
	fn div_assign(&mut self, rhs: Bound_usize<A_MIN, A_MAX>) {
		self.set(usize::saturating_div(self.get(), rhs.get()));
	}
}

#[test]
fn test_div_assign() {
	let mut bound = Bound_usize::<0, 100>::new(100);
	bound /= Bound_usize::<5, 20>::new(2);
	assert_eq!(bound.get(), 20);
	bound /= Bound_usize::<6, 20>::new(0);
	assert_eq!(bound.get(), 3);
	bound /= Bound_usize::<20, 50>::new(100);
	assert_eq!(bound.get(), 0);
}

#[test]
#[should_panic]
fn test_div_assign_panic() {
	let mut bound = Bound_usize::<0, 100>::new(50);
	bound /= Bound_usize::<0, 50>::new(0);
}
