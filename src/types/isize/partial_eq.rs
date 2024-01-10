use crate::prelude::Bound_isize;

impl<const A_MIN: isize, const A_MAX: isize, const B_MIN: isize, const B_MAX: isize> PartialEq<Bound_isize<A_MIN, A_MAX>> for Bound_isize<B_MIN, B_MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn eq(&self, other: &Bound_isize<A_MIN, A_MAX>) -> bool {
		return self.inner == other.inner;
	}
}

#[cfg(test)]
pub mod tests {
	use crate::prelude::Bound_isize;
	use crate::types::test_macros::partial_eq::{test_signed_eq, test_signed_ne};

	#[test] fn test_eq() { test_signed_eq!(Bound_isize); }
	#[test] fn test_ne() { test_signed_ne!(Bound_isize); }
}