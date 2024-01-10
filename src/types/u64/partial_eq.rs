use crate::prelude::Bound_u64;

impl<const A_MIN: u64, const A_MAX: u64, const B_MIN: u64, const B_MAX: u64> PartialEq<Bound_u64<A_MIN, A_MAX>> for Bound_u64<B_MIN, B_MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn eq(&self, other: &Bound_u64<A_MIN, A_MAX>) -> bool {
		return self.inner == other.inner;
	}
}

#[cfg(test)]
pub mod tests {
	use crate::prelude::Bound_u64;
	use crate::types::test_macros::partial_eq::{test_unsigned_eq, test_unsigned_ne};

	#[test] fn test_eq() { test_unsigned_eq!(Bound_u64); }
	#[test] fn test_ne() { test_unsigned_ne!(Bound_u64); }
}