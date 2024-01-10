use crate::prelude::{Bound_i16, SqueezeTo_i16};

impl<T, const MIN: i16, const MAX: i16> From<T> for Bound_i16<MIN, MAX> where T: SqueezeTo_i16 {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn from(inner: T) -> Self {
		return Self::new(inner.squeeze_to_i16());
	}
}

#[test]
fn test_from() {
	use crate::types::test_macros::from::test_signed_from;
	
	test_signed_from!(Bound_i16);
}