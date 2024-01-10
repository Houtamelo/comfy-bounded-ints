use crate::prelude::{Bound_u64, SqueezeTo_u64};

impl<T, const MIN: u64, const MAX: u64> From<T> for Bound_u64<MIN, MAX> where T: SqueezeTo_u64 {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn from(inner: T) -> Self {
		return Self::new(inner.squeeze_to_u64());
	}
}

#[test]
fn test_from() {
	use crate::types::test_macros::from::test_unsigned_from;
	
	test_unsigned_from!(Bound_u64);
}