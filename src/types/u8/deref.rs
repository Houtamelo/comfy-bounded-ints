use std::ops::Deref;
use crate::prelude::Bound_u8;

impl<const MIN: u8, const MAX: u8> Deref for Bound_u8<MIN, MAX> {
	type Target = u8;

	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn deref(&self) -> &Self::Target {
		return &self.inner;
	}
}

#[test]
fn test_deref() {
	use crate::types::test_macros::deref::test_unsigned_deref;

	test_unsigned_deref!(Bound_u8);
}