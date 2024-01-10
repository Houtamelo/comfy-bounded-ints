use crate::prelude::{Bound_isize, SqueezeTo_isize};

impl<T, const MIN: isize, const MAX: isize> From<T> for Bound_isize<MIN, MAX> where T: SqueezeTo_isize {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn from(inner: T) -> Self {
		return Self::new(inner.squeeze_to_isize());
	}
}

#[test]
fn test_from() {
	use crate::types::test_macros::from::test_signed_from;
	
	test_signed_from!(Bound_isize);
}