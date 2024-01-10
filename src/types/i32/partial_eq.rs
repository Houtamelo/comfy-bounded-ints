use crate::prelude::Bound_i32;

impl<const A_MIN: i32, const A_MAX: i32, const B_MIN: i32, const B_MAX: i32> PartialEq<Bound_i32<A_MIN, A_MAX>> for Bound_i32<B_MIN, B_MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn eq(&self, other: &Bound_i32<A_MIN, A_MAX>) -> bool {
		return self.inner == other.inner;
	}
}

#[cfg(test)]
pub mod tests {
	use crate::prelude::Bound_i32;
	use crate::types::test_macros::partial_eq::{test_signed_eq, test_signed_ne};

	#[test] fn test_eq() { test_signed_eq!(Bound_i32); }
	#[test] fn test_ne() { test_signed_ne!(Bound_i32); }
}