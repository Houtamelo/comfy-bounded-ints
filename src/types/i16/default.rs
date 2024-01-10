use crate::prelude::Bound_i16;

impl<const MIN: i16, const MAX: i16> Default for Bound_i16<MIN, MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn default() -> Self {
		return Self::new(MIN);
	}
}

#[test]
fn test_default() {
	use crate::types::test_macros::default::test_signed_default;
	
	test_signed_default!(Bound_i16);
}