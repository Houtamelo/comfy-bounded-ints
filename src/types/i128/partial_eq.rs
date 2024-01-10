use crate::prelude::Bound_i128;

impl<const A_MIN: i128, const A_MAX: i128, const B_MIN: i128, const B_MAX: i128> PartialEq<Bound_i128<A_MIN, A_MAX>> for Bound_i128<B_MIN, B_MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn eq(&self, other: &Bound_i128<A_MIN, A_MAX>) -> bool {
		return self.inner == other.inner;
	}
}

#[cfg(test)]
pub mod tests {
	use crate::prelude::Bound_i128;
	use crate::types::test_macros::partial_eq::{test_signed_eq, test_signed_ne};

	#[test] fn test_eq() { test_signed_eq!(Bound_i128); }
	#[test] fn test_ne() { test_signed_ne!(Bound_i128); }
}