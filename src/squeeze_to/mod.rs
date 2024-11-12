use macros::*;

mod macros;

#[doc(hidden)]
pub trait SqueezeInto<T> {
	#[doc(hidden)]
	fn squeeze_into(self) -> T;
}

impl<A: Clone + SqueezeInto<B>, B> SqueezeInto<B> for &A {
	#[inline(always)]
	fn squeeze_into(self) -> B {
		self.clone().squeeze_into()
	}
}

impl<A: Clone + SqueezeInto<B>, B> SqueezeInto<B> for &mut A {
	#[inline(always)]
	fn squeeze_into(self) -> B {
		self.clone().squeeze_into()
	}
}

pub trait Squeeze: Sized {
	#[inline(always)]
	fn squeeze<T>(self) -> T
		where Self: SqueezeInto<T>
	{
		self.squeeze_into()
	}
}

#[doc(hidden)]
pub trait SqueezeFrom<T> {
	fn squeeze_from(value: T) -> Self;
}

impl<A, B> SqueezeFrom<B> for A
	where B: SqueezeInto<A>
{
	#[inline(always)]
	fn squeeze_from(value: B) -> Self {
		value.squeeze_into()
	}
}

impl_squeeze!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

signed_to_signed!(i8 <<< [i8, i16, i32, i64, i128, isize]);
signed_to_signed!([i8] <<< i16 <<< [i16, i32, i64, i128, isize]);
signed_to_signed!([i8, i16] <<< i32 <<< [i32, i64, i128, isize]);
signed_to_signed!([i8, i16, i32] <<< i64 <<< [i64, i128]);
signed_to_signed!([i8, i16, i32, i64, isize] <<< i128 <<< [i128]);
signed_to_signed!([i8, i16] <<< isize <<< [i64, i128, isize]);

signed_to_unsigned!(i8 <<< [u8, u16, u32, u64, u128, usize]);
signed_to_unsigned!([u8] <<< i16 <<< [u16, u32, u64, u128, usize]);
signed_to_unsigned!([u8, u16] <<< i32 <<< [u32, u64, u128, usize]);
signed_to_unsigned!([u8, u16, u32] <<< i64 <<< [u64, u128]);
signed_to_unsigned!([u8, u16, u32, u64, usize] <<< i128 <<< [u128]);
signed_to_unsigned!([u8, u16] <<< isize <<< [u64, u128, usize]);

unsigned!([i8] <<< u8 <<< [i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize]);
unsigned!([i8, i16, u8] <<< u16 <<< [i32, i64, i128, isize, u16, u32, u64, u128, usize]);
unsigned!([i8, i16, i32, u8, u16] <<< u32 <<< [i64, i128, u32, u64, u128, usize]);
unsigned!([i8, i16, i32, i64, isize, u8, u16, u32] <<< u64 <<< [i128, u64, u128]);
unsigned!([i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, usize] <<< u128 <<< [u128]);
unsigned!([i8, i16, i32, isize, u8, u16] <<< usize <<< [i128, u64, u128, usize]);

#[cfg(target_pointer_width = "32")]
mod ptr_32 {
	use super::*;

	signed_to_signed!([isize] <<< i64);
	signed_to_unsigned!([usize] <<< i64);
	
	signed_to_signed!(isize <<< [i32]);
	signed_to_unsigned!(isize <<< [u32]);

	unsigned!([isize] <<< u32);
	unsigned!([usize] <<< u64);
	unsigned!(usize <<< [i64, u32]);
}

#[cfg(target_pointer_width = "64")]
mod ptr_64 {
	use super::*;

	signed_to_signed!(i64 <<< [isize]);
	signed_to_unsigned!(i64 <<< [usize]);
	
	signed_to_signed!([i32] <<< isize);
	signed_to_unsigned!([u32] <<< isize);

	unsigned!(u32 <<< [isize]);
	unsigned!(u64 <<< [usize]);
	unsigned!([i64, u32] <<< usize);
}