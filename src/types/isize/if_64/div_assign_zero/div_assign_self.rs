use std::ops::DivAssign;

use crate::prelude::Bound_isize;

impl<const A_MIN: isize, const A_MAX: isize, const B_MIN: isize, const B_MAX: isize> DivAssign<Bound_isize<A_MIN, A_MAX>> for Bound_isize<B_MIN, B_MAX> {
	/// # Panics
	/// If rhs is 0.
	/// 
	/// Does not panic on overflowing, this is saturated.
	#[inline(always)]
	fn div_assign(&mut self, rhs: Bound_isize<A_MIN, A_MAX>) {
		self.set(isize::saturating_div(self.get(), rhs.get()));
	}
}

#[test]
fn test_div_assign() {
	let mut bound = Bound_isize::<-100, 100>::new(100);
	bound /= Bound_isize::<-50, 20>::new(2);
	assert_eq!(bound.get(), 50);
	bound /= Bound_isize::<-3, 20>::new(-30);
	assert_eq!(bound.get(), -16);
	bound /= Bound_isize::<-50, 2>::new(50);
	assert_eq!(bound.get(), -8);
	bound /= Bound_isize::<-50, 127>::new(100);
	assert_eq!(bound.get(), 0);
	
	const MIN: isize = isize::MIN;
	let mut bound = Bound_isize::<MIN, 100>::new(MIN);
	bound /= Bound_isize::<-50, 20>::new(-1);
	assert_eq!(bound.get(), 100);
}

#[test]
#[should_panic]
fn test_div_assign_panic() {
	let mut bound = Bound_isize::<-100, 100>::new(50);
	bound /= Bound_isize::<-100, 100>::new(0);
}
