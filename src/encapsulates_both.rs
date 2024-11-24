use std::ops::*;

#[doc(hidden)]
pub trait EncapsulatesBoth<Other> {
	type Out: Add<Output = Self::Out>
		+ Sub<Output = Self::Out>
		+ Mul<Output = Self::Out>
		+ Div<Output = Self::Out>
		+ Rem<Output = Self::Out>;
}

macro_rules! encapsulates_both {
    ($(
        ($A: ty, $B: ty) => $Bridge: ty
    ),* $(,)?) => {
	    $(
	        impl EncapsulatesBoth<$B> for $A { type Out = $Bridge; }
	        impl EncapsulatesBoth<&$B> for $A { type Out = $Bridge; }
	        impl EncapsulatesBoth<&mut $B> for $A { type Out = $Bridge; }

	        impl EncapsulatesBoth<$B> for &$A { type Out = $Bridge; }
	        impl EncapsulatesBoth<&$B> for &$A { type Out = $Bridge; }
	        impl EncapsulatesBoth<&mut $B> for &$A { type Out = $Bridge; }

	        impl EncapsulatesBoth<$B> for &mut $A { type Out = $Bridge; }
	        impl EncapsulatesBoth<&$B> for &mut $A { type Out = $Bridge; }
	        impl EncapsulatesBoth<&mut $B> for &mut $A { type Out = $Bridge; }

	        impl EncapsulatesBoth<$A> for $B { type Out = $Bridge; }
	        impl EncapsulatesBoth<&$A> for $B { type Out = $Bridge; }
	        impl EncapsulatesBoth<&mut $A> for $B { type Out = $Bridge; }

	        impl EncapsulatesBoth<$A> for &$B { type Out = $Bridge; }
	        impl EncapsulatesBoth<&$A> for &$B { type Out = $Bridge; }
	        impl EncapsulatesBoth<&mut $A> for &$B { type Out = $Bridge; }

	        impl EncapsulatesBoth<$A> for &mut $B { type Out = $Bridge; }
	        impl EncapsulatesBoth<&$A> for &mut $B { type Out = $Bridge; }
	        impl EncapsulatesBoth<&mut $A> for &mut $B { type Out = $Bridge; }
	    )*
    };
}

encapsulates_both!(
	(i8, i16) => i16,
	(i8, i32) => i32,
	(i8, i64) => i64,
	(i8, i128) => i128,
	(i8, isize) => isize,

	(i16, i32) => i32,
	(i16, i64) => i64,
	(i16, i128) => i128,
	(i16, isize) => isize,

	(i32, i64) => i64,
	(i32, i128) => i128,
	(i32, isize) => isize,

	(i64, i128) => i128,
	(i64, isize) => i64,

	(i8, u8) => i16,
	(i8, u16) => i32,
	(i8, u32) => i64,
	(i8, u64) => i128,
	(i8, u128) => i128,

	(i16, u8) => i16,
	(i16, u16) => i32,
	(i16, u32) => i64,
	(i16, u64) => i128,
	(i16, u128) => i128,

	(i32, u8) => i32,
	(i32, u16) => i32,
	(i32, u32) => i64,
	(i32, u64) => i128,
	(i32, u128) => i128,

	(i64, u8) => i64,
	(i64, u16) => i64,
	(i64, u32) => i64,
	(i64, u64) => i128,
	(i64, u128) => i128,

	(i128, u8) => i128,
	(i128, u16) => i128,
	(i128, u32) => i128,
	(i128, u64) => i128,
	(i128, u128) => i128,
	(i128, usize) => i128,

	(isize, u8) => isize,
	(isize, u16) => isize,
	(isize, u32) => i64,
	(isize, u64) => i128,
	(isize, u128) => i128,

	(u8, u16) => u16,
	(u8, u32) => u32,
	(u8, u64) => u64,
	(u8, u128) => u128,
	(u8, usize) => usize,

	(u16, u32) => u32,
	(u16, u64) => u64,
	(u16, u128) => u128,
	(u16, usize) => usize,

	(u32, u64) => u64,
	(u32, u128) => u128,
	(u32, usize) => usize,

	(u64, u128) => u128,
	(u64, usize) => u64,

	(u128, usize) => usize,
);

#[cfg(target_pointer_width = "32")]
mod ptr_32 {
	use super::*;

	encapsulates_both!(
		(i8, usize) => i64,
		(i16, usize) => i64,
		(i32, usize) => i64,
		(i64, usize) => i64,
		(isize, usize) => i64,
	);
}

#[cfg(target_pointer_width = "64")]
mod ptr_64 {
	use super::*;

	encapsulates_both!(
		(i8, usize) => i128,
		(i16, usize) => i128,
		(i32, usize) => i128,
		(i64, usize) => i128,
		(isize, usize) => i128,
	);
}

macro_rules! encapsulates_self {
    ($($T: ty),* $(,)?) => {
	    $(
	        impl EncapsulatesBoth<$T> for $T { type Out = $T; }
	        impl EncapsulatesBoth<&$T> for $T { type Out = $T; }
	        impl EncapsulatesBoth<&mut $T> for $T { type Out = $T; }

	        impl EncapsulatesBoth<$T> for &$T { type Out = $T; }
	        impl EncapsulatesBoth<&$T> for &$T { type Out = $T; }
	        impl EncapsulatesBoth<&mut $T> for &$T { type Out = $T; }

	        impl EncapsulatesBoth<$T> for &mut $T { type Out = $T; }
	        impl EncapsulatesBoth<&$T> for &mut $T { type Out = $T; }
	        impl EncapsulatesBoth<&mut $T> for &mut $T { type Out = $T; }
	    )*
    };
}

encapsulates_self!(
	i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize,
);
