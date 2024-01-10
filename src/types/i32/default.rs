use crate::prelude::Bound_i32;

impl<const MIN: i32, const MAX: i32> Default for Bound_i32<MIN, MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn default() -> Self {
		return Self::new(MIN);
	}
}

#[test]
fn test_default() {
	use crate::types::test_macros::default::test_signed_default;
	
	test_signed_default!(Bound_i32);
}