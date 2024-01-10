use crate::prelude::Bound_i8;

impl<const MIN: i8, const MAX: i8> Default for Bound_i8<MIN, MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn default() -> Self {
		return Self::new(MIN);
	}
}

#[test]
fn test_default() {
	use crate::types::test_macros::default::test_signed_default;
	
	test_signed_default!(Bound_i8);
}
