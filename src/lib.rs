#![feature(macro_metavar_expr)]
#![feature(macro_metavar_expr_concat)]
#![feature(negative_impls)]
#![feature(auto_traits)]
#![feature(step_trait)]
#![allow(non_camel_case_types)]
#![allow(clippy::derived_hash_with_manual_eq)]
#![allow(clippy::derive_ord_xor_partial_ord)]
#![doc = include_str!("../README.md")]

mod cram;
mod encapsulates_both;
mod macros;
mod saturated;

#[doc(hidden)]
pub auto trait IPrimitive {}
impl<T: IPrimitive> IPrimitive for &T {}
impl<T: IPrimitive> IPrimitive for &mut T {}

#[doc(hidden)]
pub auto trait INotRef {}
impl<T> !INotRef for &T {}
impl<T> !INotRef for &mut T {}

#[doc(hidden)]
pub use encapsulates_both::EncapsulatesBoth;

#[cfg(test)] mod tests;

#[allow(unused)]
pub mod prelude {
	pub use super::{
		cram::{ClampRange, CramFrom, CramInto, cram},
		saturated::{Int, UInt},
	};
	pub use crate::{
		new_bound_signed,
		new_bound_unsigned,
		new_generic_bound_signed,
		new_generic_bound_unsigned,
	};
}
