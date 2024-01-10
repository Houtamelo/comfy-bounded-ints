use std::ops::DivAssign;

use crate::prelude::Bound_i8;

impl<const A_MIN: i8, const A_MAX: i8, const B_MIN: i8, const B_MAX: i8> DivAssign<Bound_i8<A_MIN, A_MAX>> for Bound_i8<B_MIN, B_MAX> {
	/// # Panics
	/// If rhs is 0.
	/// 
	/// Does not panic on overflowing, this is saturated.
	#[inline(always)]
	fn div_assign(&mut self, rhs: Bound_i8<A_MIN, A_MAX>) {
		self.set(i8::saturating_div(self.get(), rhs.get()));
	}
}

#[test]
fn test_div_assign() {
	let mut bound = Bound_i8::<-100, 100>::new(100);
	bound /= Bound_i8::<-50, 20>::new(2);
	assert_eq!(bound.get(), 50);
	bound /= Bound_i8::<-3, 20>::new(-30);
	assert_eq!(bound.get(), -16);
	bound /= Bound_i8::<-50, 2>::new(50);
	assert_eq!(bound.get(), -8);
	bound /= Bound_i8::<-50, 127>::new(100);
	assert_eq!(bound.get(), 0);
	
	const MIN: i8 = i8::MIN;
	let mut bound = Bound_i8::<MIN, 100>::new(MIN);
	bound /= Bound_i8::<-50, 20>::new(-1);
	assert_eq!(bound.get(), 100);
}

#[test]
#[should_panic]
fn test_div_assign_panic() {
	let mut bound = Bound_i8::<-100, 100>::new(50);
	bound /= Bound_i8::<-100, 100>::new(0);
}
