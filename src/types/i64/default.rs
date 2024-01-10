use crate::prelude::Bound_i64;

impl<const MIN: i64, const MAX: i64> Default for Bound_i64<MIN, MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn default() -> Self {
		return Self::new(MIN);
	}
}

#[test]
fn test_default() {
	use crate::types::test_macros::default::test_signed_default;
	
	test_signed_default!(Bound_i64);
}