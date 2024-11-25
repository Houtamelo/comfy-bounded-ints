#![feature(macro_metavar_expr)]
#![feature(macro_metavar_expr_concat)]
#![feature(negative_impls)]
#![feature(auto_traits)]
#![allow(non_camel_case_types)]
#![allow(clippy::derived_hash_with_manual_eq)]
#![allow(clippy::derive_ord_xor_partial_ord)]
#![doc = include_str!("../README.md")]

mod bounded;
mod cram;
mod encapsulates_both;
mod macros;
mod saturated;

pub auto trait IPrimitive {}
impl<T: IPrimitive> IPrimitive for &T {}
impl<T: IPrimitive> IPrimitive for &mut T {}

pub auto trait INotRef {}
impl<T> !INotRef for &T {}
impl<T> !INotRef for &mut T {}

#[cfg(test)] mod tests;

#[allow(unused)]
pub mod prelude {
	pub type Int = Sat_i64;
	pub type UInt = Sat_u64;

	pub use super::{
		bounded::{
			Bnd_i8,
			Bnd_i16,
			Bnd_i32,
			Bnd_i64,
			Bnd_i128,
			Bnd_isize,
			Bnd_u8,
			Bnd_u16,
			Bnd_u32,
			Bnd_u64,
			Bnd_u128,
			Bnd_usize,
		},
		cram::{CramFrom, CramInto, clamp, cram},
		saturated::{
			Sat_i8,
			Sat_i16,
			Sat_i32,
			Sat_i64,
			Sat_i128,
			Sat_isize,
			Sat_u8,
			Sat_u16,
			Sat_u32,
			Sat_u64,
			Sat_u128,
			Sat_usize,
		},
	};
	#[doc(hidden)]
	pub use crate::IPrimitive;
	#[doc(hidden)]
	pub use crate::encapsulates_both::EncapsulatesBoth;
	#[doc(hidden)]
	pub use crate::{
		impl_basic_ops,
		impl_basic_ops_assign,
		impl_cmp,
		impl_conversions,
		impl_deref,
		impl_display,
		impl_neg,
	};
	pub use crate::{
		new_bound_signed,
		new_bound_unsigned,
		new_generic_bound_signed,
		new_generic_bound_unsigned,
	};
}
