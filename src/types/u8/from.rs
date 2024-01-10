use crate::prelude::{Bound_u8, SqueezeTo_u8};

impl<T, const MIN: u8, const MAX: u8> From<T> for Bound_u8<MIN, MAX> where T: SqueezeTo_u8 {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn from(inner: T) -> Self {
		return Self::new(inner.squeeze_to_u8());
	}
}

#[test]
fn test_from() {
	use crate::types::test_macros::from::test_unsigned_from;
	
	test_unsigned_from!(Bound_u8);
}