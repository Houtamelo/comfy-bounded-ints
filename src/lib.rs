#![allow(non_camel_case_types)]
#![allow(clippy::needless_return)]

pub mod pad_to;
pub mod squeeze_to;
pub mod types;

pub mod prelude {
	pub use super::pad_to::*;
	pub use super::squeeze_to::*;
	pub use super::types::*;
}