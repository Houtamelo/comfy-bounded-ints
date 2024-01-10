use crate::prelude::Bound_i64;

impl<const A_MIN: i64, const A_MAX: i64, const B_MIN: i64, const B_MAX: i64> PartialEq<Bound_i64<A_MIN, A_MAX>> for Bound_i64<B_MIN, B_MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn eq(&self, other: &Bound_i64<A_MIN, A_MAX>) -> bool {
		return self.inner == other.inner;
	}
}

#[cfg(test)]
pub mod tests {
	use crate::prelude::Bound_i64;
	use crate::types::test_macros::partial_eq::{test_signed_eq, test_signed_ne};

	#[test] fn test_eq() { test_signed_eq!(Bound_i64); }
	#[test] fn test_ne() { test_signed_ne!(Bound_i64); }
}