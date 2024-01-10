use std::ops::Deref;
use crate::prelude::Bound_i32;

impl<const MIN: i32, const MAX: i32> Deref for Bound_i32<MIN, MAX> {
	type Target = i32;

	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn deref(&self) -> &Self::Target {
		return &self.inner;
	}
}

#[test]
fn test_deref() {
	use crate::types::test_macros::deref::test_signed_deref;
	
	test_signed_deref!(Bound_i32);
}