use crate::prelude::{Bound_u32, SqueezeTo_u32};

impl<T, const MIN: u32, const MAX: u32> From<T> for Bound_u32<MIN, MAX> where T: SqueezeTo_u32 {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn from(inner: T) -> Self {
		return Self::new(inner.squeeze_to_u32());
	}
}

#[test]
fn test_from() {
	use crate::types::test_macros::from::test_unsigned_from;
	
	test_unsigned_from!(Bound_u32);
}