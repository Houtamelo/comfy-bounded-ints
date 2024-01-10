use crate::prelude::Bound_u32;

impl<const MIN: u32, const MAX: u32> Default for Bound_u32<MIN, MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn default() -> Self {
		return Self::new(MIN);
	}
}

#[test]
fn test_default() {
	use crate::types::test_macros::default::test_unsigned_default;
	
	test_unsigned_default!(Bound_u32);
}