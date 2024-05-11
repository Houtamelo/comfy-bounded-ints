#![allow(non_camel_case_types)]
#![allow(clippy::needless_return)]

pub mod pad_to;
pub mod squeeze_to;
pub mod types;

pub mod prelude {
	pub use super::pad_to::*;
	pub use super::saturated_types::*;
	pub use super::squeeze_to::*;
	pub use super::types::*;
}

mod saturated_types {
	use crate::prelude::*;

	pub type SaturatedU8 = Bound_u8<{ u8::MIN }, { u8::MAX }>;
	pub type SaturatedI8 = Bound_i8<{ i8::MIN }, { i8::MAX }>;
	
	pub type SaturatedU16 = Bound_u16<{ u16::MIN }, { u16::MAX }>;
	pub type SaturatedI16 = Bound_i16<{ i16::MIN }, { i16::MAX }>;
	
	pub type SaturatedU32 = Bound_u32<{ u32::MIN }, { u32::MAX }>;
	pub type SaturatedI32 = Bound_i32<{ i32::MIN }, { i32::MAX }>;

	pub type SaturatedU64 = Bound_u64<{ u64::MIN }, { u64::MAX }>;
	pub type SaturatedI64 = Bound_i64<{ i64::MIN }, { i64::MAX }>;
	
	pub type SaturatedU128 = Bound_u128<{ u128::MIN }, { u128::MAX }>;
	pub type SaturatedI128 = Bound_i128<{ i128::MIN }, { i128::MAX }>;
	
	pub type SaturatedUSize = Bound_usize<{ usize::MIN }, { usize::MAX }>;
	pub type SaturatedISize = Bound_isize<{ isize::MIN }, { isize::MAX }>;
}