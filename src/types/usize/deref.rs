use std::ops::Deref;
use crate::prelude::Bound_usize;

impl<const MIN: usize, const MAX: usize> Deref for Bound_usize<MIN, MAX> {
	type Target = usize;

	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn deref(&self) -> &Self::Target {
		return &self.inner;
	}
}

#[test]
fn test_deref() {
	use crate::types::test_macros::deref::test_unsigned_deref;

	test_unsigned_deref!(Bound_usize);
}