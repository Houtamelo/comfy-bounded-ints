use std::ops::{Rem, RemAssign};

use crate::prelude::Bound_i64;

impl<const A_MIN: i64, const A_MAX: i64, const B_MIN: i64, const B_MAX: i64> RemAssign<Bound_i64<A_MIN, A_MAX>> for Bound_i64<B_MIN, B_MAX> {
	/// # Panics
	/// If rhs is 0.
	/// 
	/// Does not panic on overflowing, this is saturated.
	#[inline(always)]
	fn rem_assign(&mut self, rhs: Bound_i64<A_MIN, A_MAX>) {
		if (self.get() == i64::MIN) && (rhs.get() == -1) {
			self.set(0);
		} else {
			self.set(i64::rem(self.get(), rhs.get()));
		}
	}
}

#[test]
fn test_rem_assign() {
	let mut bound = Bound_i64::<-100, 100>::new(100);
	bound %= Bound_i64::<-50, 30>::new(40);
	assert_eq!(bound.get(), 10);
	bound %= Bound_i64::<-3, 20>::new(-5);
	assert_eq!(bound.get(), 1);
	bound %= Bound_i64::<-50, 2>::new(50);
	assert_eq!(bound.get(), 1);
	bound %= Bound_i64::<-50, 127>::new(-1);
	assert_eq!(bound.get(), 0);
	
	const MIN: i64 = i64::MIN;
	let mut bound = Bound_i64::<MIN, 100>::new(MIN);
	bound %= Bound_i64::<-50, 20>::new(-1);
	assert_eq!(bound.get(), 0);
}

#[test]
#[should_panic]
fn test_rem_assign_panic() {
	let mut bound = Bound_i64::<-100, 100>::new(100);
	bound %= Bound_i64::<-50, 127>::new(0);
}
