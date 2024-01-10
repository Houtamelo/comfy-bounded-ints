use std::ops::{Rem, RemAssign};

use crate::prelude::Bound_i16;

impl<const A_MIN: i16, const A_MAX: i16, const B_MIN: i16, const B_MAX: i16> RemAssign<Bound_i16<A_MIN, A_MAX>> for Bound_i16<B_MIN, B_MAX> {
	/// # Panics
	/// If rhs is 0.
	/// 
	/// Does not panic on overflowing, this is saturated.
	#[inline(always)]
	fn rem_assign(&mut self, rhs: Bound_i16<A_MIN, A_MAX>) {
		if (self.get() == i16::MIN) && (rhs.get() == -1) {
			self.set(0);
		} else {
			self.set(i16::rem(self.get(), rhs.get()));
		}
	}
}

#[test]
fn test_rem_assign() {
	let mut bound = Bound_i16::<-100, 100>::new(100);
	bound %= Bound_i16::<-50, 30>::new(40);
	assert_eq!(bound.get(), 10);
	bound %= Bound_i16::<-3, 20>::new(-5);
	assert_eq!(bound.get(), 1);
	bound %= Bound_i16::<-50, 2>::new(50);
	assert_eq!(bound.get(), 1);
	bound %= Bound_i16::<-50, 127>::new(-1);
	assert_eq!(bound.get(), 0);
	
	const MIN: i16 = i16::MIN;
	let mut bound = Bound_i16::<MIN, 100>::new(MIN);
	bound %= Bound_i16::<-50, 20>::new(-1);
	assert_eq!(bound.get(), 0);
}

#[test]
#[should_panic]
fn test_rem_assign_panic() {
	let mut bound = Bound_i16::<-100, 100>::new(100);
	bound %= Bound_i16::<-50, 127>::new(0);
}
