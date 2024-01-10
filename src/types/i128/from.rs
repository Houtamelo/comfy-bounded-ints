use crate::prelude::Bound_i128;

impl<const MIN: i128, const MAX: i128> From<i128> for Bound_i128<MIN, MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn from(inner: i128) -> Self {
		return Self::new(inner);
	}
}

#[test]
fn test_from() {
	use crate::types::test_macros::from::test_signed_from;
	
	test_signed_from!(Bound_i128);
}