use crate::prelude::Bound_usize;

impl<const MIN: usize, const MAX: usize> Default for Bound_usize<MIN, MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn default() -> Self {
		return Self::new(MIN);
	}
}

#[test]
fn test_default() {
	use crate::types::test_macros::default::test_unsigned_default;
	
	test_unsigned_default!(Bound_usize);
}