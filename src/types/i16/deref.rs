use std::ops::Deref;
use crate::prelude::Bound_i16;

impl<const MIN: i16, const MAX: i16> Deref for Bound_i16<MIN, MAX> {
	type Target = i16;

	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn deref(&self) -> &Self::Target {
		return &self.inner;
	}
}

#[test]
fn test_deref() {
	use crate::types::test_macros::deref::test_signed_deref;
	
	test_signed_deref!(Bound_i16);
}