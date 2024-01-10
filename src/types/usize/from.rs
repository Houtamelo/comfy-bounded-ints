use crate::prelude::{Bound_usize, SqueezeTo_usize};

impl<T, const MIN: usize, const MAX: usize> From<T> for Bound_usize<MIN, MAX> where T: SqueezeTo_usize {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn from(inner: T) -> Self {
		return Self::new(inner.squeeze_to_usize());
	}
}

#[test]
fn test_from() {
	use crate::types::test_macros::from::test_unsigned_from;
	
	test_unsigned_from!(Bound_usize);
}