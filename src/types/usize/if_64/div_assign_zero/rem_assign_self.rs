use std::ops::{Rem, RemAssign};

use crate::prelude::Bound_usize;

impl<const A_MIN: usize, const A_MAX: usize, const B_MIN: usize, const B_MAX: usize> RemAssign<Bound_usize<A_MIN, A_MAX>> for Bound_usize<B_MIN, B_MAX> {
	/// # Panics
	/// If rhs is 0.
	/// 
	/// Does not panic on overflowing, this is saturated.
	#[inline(always)]
	fn rem_assign(&mut self, rhs: Bound_usize<A_MIN, A_MAX>) {
		self.set(usize::rem(self.get(), rhs.get()));
	}
}

#[test]
fn test_rem_assign() {
	let mut bound = Bound_usize::<0, 100>::new(100);
	bound %= Bound_usize::<10, 30>::new(40);
	assert_eq!(bound.get(), 10);
	bound %= Bound_usize::<5, 20>::new(0);
	assert_eq!(bound.get(), 0);
	
	bound.set(80);
	bound %= Bound_usize::<0, 3>::new(50);
	assert_eq!(bound.get(), 2);
	bound %= Bound_usize::<0, 127>::new(50);
	assert_eq!(bound.get(), 2);
}

#[test]
#[should_panic]
fn test_rem_assign_panic() {
	let mut bound = Bound_usize::<0, 100>::new(60);
	bound %= Bound_usize::<0, 127>::new(0);
}
