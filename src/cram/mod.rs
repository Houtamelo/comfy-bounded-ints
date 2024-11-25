use macros::*;
mod macros;

pub fn clamp<T: PartialOrd>(value: impl CramInto<T>, range: std::ops::Range<T>) -> T {
	let value = value.cram_into();
	if value < range.start {
		range.start
	} else if value > range.end {
		range.end
	} else {
		value
	}
}

pub fn cram<T>(value: impl CramInto<T>) -> T { value.cram_into() }

#[doc(hidden)]
pub trait CramInto<T> {
	#[doc(hidden)]
	fn cram_into(self) -> T;
}

#[doc(hidden)]
pub trait CramFrom<T> {
	fn cram_from(value: T) -> Self;
}

impl<A, B> CramFrom<B> for A
where B: CramInto<A>
{
	#[inline(always)]
	fn cram_from(value: B) -> Self { value.cram_into() }
}

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
