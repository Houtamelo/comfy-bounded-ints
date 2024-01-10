use crate::prelude::{Bound_i32, SqueezeTo_i32};

impl<T, const MIN: i32, const MAX: i32> From<T> for Bound_i32<MIN, MAX> where T: SqueezeTo_i32 {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn from(inner: T) -> Self {
		return Self::new(inner.squeeze_to_i32());
	}
}

#[test]
fn test_from() {
	use crate::types::test_macros::from::test_signed_from;
	
	test_signed_from!(Bound_i32);
}