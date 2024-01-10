use crate::prelude::Bound_u16;

impl<const A_MIN: u16, const A_MAX: u16, const B_MIN: u16, const B_MAX: u16> PartialEq<Bound_u16<A_MIN, A_MAX>> for Bound_u16<B_MIN, B_MAX> {
	#[inline(always)]
	#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
	fn eq(&self, other: &Bound_u16<A_MIN, A_MAX>) -> bool {
		return self.inner == other.inner;
	}
}

#[cfg(test)]
pub mod tests {
	use crate::prelude::Bound_u16;
	use crate::types::test_macros::partial_eq::{test_unsigned_eq, test_unsigned_ne};

	#[test] fn test_eq() { test_unsigned_eq!(Bound_u16); }
	#[test] fn test_ne() { test_unsigned_ne!(Bound_u16); }
}