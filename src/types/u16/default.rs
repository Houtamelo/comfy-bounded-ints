use crate::prelude::Bound_u16;

impl<const MIN: u16, const MAX: u16> Default for Bound_u16<MIN, MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn default() -> Self {
		return Self::new(MIN);
	}
}

#[test]
fn test_default() {
	use crate::types::test_macros::default::test_unsigned_default;
	
	test_unsigned_default!(Bound_u16);
}