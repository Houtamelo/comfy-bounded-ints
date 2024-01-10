use std::ops::Deref;
use crate::prelude::Bound_u64;

impl<const MIN: u64, const MAX: u64> Deref for Bound_u64<MIN, MAX> {
	type Target = u64;

	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn deref(&self) -> &Self::Target {
		return &self.inner;
	}
}

#[test]
fn test_deref() {
	use crate::types::test_macros::deref::test_unsigned_deref;

	test_unsigned_deref!(Bound_u64);
}