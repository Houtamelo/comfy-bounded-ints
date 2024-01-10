use crate::prelude::Bound_u8;

impl<const A_MIN: u8, const A_MAX: u8, const B_MIN: u8, const B_MAX: u8> PartialEq<Bound_u8<A_MIN, A_MAX>> for Bound_u8<B_MIN, B_MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn eq(&self, other: &Bound_u8<A_MIN, A_MAX>) -> bool {
		return self.inner == other.inner;
	}
}

#[cfg(test)]
pub mod tests {
	use crate::prelude::Bound_u8;
	use crate::types::test_macros::partial_eq::{test_unsigned_eq, test_unsigned_ne};

	#[test] fn test_eq() { test_unsigned_eq!(Bound_u8); }
	#[test] fn test_ne() { test_unsigned_ne!(Bound_u8); }
}