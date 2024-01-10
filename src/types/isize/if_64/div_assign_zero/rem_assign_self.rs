use std::ops::{Rem, RemAssign};

use crate::prelude::Bound_isize;

impl<const A_MIN: isize, const A_MAX: isize, const B_MIN: isize, const B_MAX: isize> RemAssign<Bound_isize<A_MIN, A_MAX>> for Bound_isize<B_MIN, B_MAX> {
	/// # Panics
	/// If rhs is 0.
	/// 
	/// Does not panic on overflowing, this is saturated.
	#[inline(always)]
	fn rem_assign(&mut self, rhs: Bound_isize<A_MIN, A_MAX>) {
		if (self.get() == isize::MIN) && (rhs.get() == -1) {
			self.set(0);
		} else {
			self.set(isize::rem(self.get(), rhs.get()));
		}
	}
}

#[test]
fn test_rem_assign() {
	let mut bound = Bound_isize::<-100, 100>::new(100);
	bound %= Bound_isize::<-50, 30>::new(40);
	assert_eq!(bound.get(), 10);
	bound %= Bound_isize::<-3, 20>::new(-5);
	assert_eq!(bound.get(), 1);
	bound %= Bound_isize::<-50, 2>::new(50);
	assert_eq!(bound.get(), 1);
	bound %= Bound_isize::<-50, 127>::new(-1);
	assert_eq!(bound.get(), 0);
	
	const MIN: isize = isize::MIN;
	let mut bound = Bound_isize::<MIN, 100>::new(MIN);
	bound %= Bound_isize::<-50, 20>::new(-1);
	assert_eq!(bound.get(), 0);
}

#[test]
#[should_panic]
fn test_rem_assign_panic() {
	let mut bound = Bound_isize::<-100, 100>::new(100);
	bound %= Bound_isize::<-50, 127>::new(0);
}
