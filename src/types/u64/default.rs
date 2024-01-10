use crate::prelude::Bound_u64;

impl<const MIN: u64, const MAX: u64> Default for Bound_u64<MIN, MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn default() -> Self {
		return Self::new(MIN);
	}
}

#[test]
fn test_default() {
	use crate::types::test_macros::default::test_unsigned_default;
	
	test_unsigned_default!(Bound_u64);
}