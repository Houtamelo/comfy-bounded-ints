use std::ops::Deref;
use crate::prelude::Bound_isize;

impl<const MIN: isize, const MAX: isize> Deref for Bound_isize<MIN, MAX> {
	type Target = isize;

	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn deref(&self) -> &Self::Target {
		return &self.inner;
	}
}

#[test]
fn test_deref() {
	use crate::types::test_macros::deref::test_signed_deref;
	
	test_signed_deref!(Bound_isize);
}