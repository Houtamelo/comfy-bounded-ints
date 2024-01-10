use crate::prelude::Bound_u128;

impl<const MIN: u128, const MAX: u128> From<u128> for Bound_u128<MIN, MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn from(inner: u128) -> Self {
		return Self::new(inner);
	}
}

#[test]
fn test_from() {
	use crate::types::test_macros::from::test_unsigned_from;
	
	test_unsigned_from!(Bound_u128);
}