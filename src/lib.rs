#![feature(macro_metavar_expr)]
#![feature(macro_metavar_expr_concat)]
#![allow(non_camel_case_types)]
#![allow(clippy::derived_hash_with_manual_eq)]
#![allow(clippy::derive_ord_xor_partial_ord)]

mod squeeze_to;
mod saturated;
mod bounded;
mod macros;

#[cfg(test)]
mod tests;

#[allow(unused)]
pub mod prelude {
	pub use super::squeeze_to::*;
	pub use super::saturated::*;
	pub use super::bounded::*;
	pub use crate::*;
}
