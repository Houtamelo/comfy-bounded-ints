use crate::prelude::{Bound_i8, SqueezeTo_i8};

impl<T, const MIN: i8, const MAX: i8> From<T> for Bound_i8<MIN, MAX> where T: SqueezeTo_i8 {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn from(inner: T) -> Self {
		return Self::new(inner.squeeze_to_i8());
	}
}

#[test]
fn test_from() {
	use crate::types::test_macros::from::test_signed_from;
	
	test_signed_from!(Bound_i8);
}
