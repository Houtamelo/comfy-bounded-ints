use crate::prelude::{Bound_u16, SqueezeTo_u16};

impl<T, const MIN: u16, const MAX: u16> From<T> for Bound_u16<MIN, MAX> where T: SqueezeTo_u16 {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn from(inner: T) -> Self {
		return Self::new(inner.squeeze_to_u16());
	}
}

#[test]
fn test_from() {
	use crate::types::test_macros::from::test_unsigned_from;
	
	test_unsigned_from!(Bound_u16);
}