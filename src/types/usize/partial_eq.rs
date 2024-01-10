use crate::prelude::Bound_usize;

impl<const A_MIN: usize, const A_MAX: usize, const B_MIN: usize, const B_MAX: usize> PartialEq<Bound_usize<A_MIN, A_MAX>> for Bound_usize<B_MIN, B_MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn eq(&self, other: &Bound_usize<A_MIN, A_MAX>) -> bool {
		return self.inner == other.inner;
	}
}

#[cfg(test)]
pub mod tests {
	use crate::prelude::Bound_usize;
	use crate::types::test_macros::partial_eq::{test_unsigned_eq, test_unsigned_ne};

	#[test] fn test_eq() { test_unsigned_eq!(Bound_usize); }
	#[test] fn test_ne() { test_unsigned_ne!(Bound_usize); }
}