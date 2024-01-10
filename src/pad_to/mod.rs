pub use pad_i128::PadTo_i128;
pub use pad_i16::PadTo_i16;
pub use pad_i32::PadTo_i32;
pub use pad_i64::PadTo_i64;
pub use pad_isize::PadTo_isize;
pub use pad_u128::PadTo_u128;
pub use pad_u16::PadTo_u16;
pub use pad_u32::PadTo_u32;
pub use pad_u64::PadTo_u64;
pub use pad_usize::PadTo_usize;

mod pad_i16;
mod pad_i32;
mod pad_i64;
mod pad_i128;
mod pad_isize;

mod pad_u16;
mod pad_u32;
mod pad_u64;
mod pad_u128;
mod pad_usize;

pub trait PadTo<T> {
	fn pad_to(&self) -> T;
}

pub mod gen_i16 {
	use super::*;

	impl<T> PadTo<i16> for T where T: PadTo_i16 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to(&self) -> i16 {
			return self.pad_to_i16();
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i16() {
			inner(127_i8, 127);
			inner(-128_i8, -128);

			inner(32767_i16, 32767);
			inner(-32768_i16, -32768);
			
			inner(0_u8, 0);
			inner(255_u8, 255);
		}
		
		fn inner(input: impl PadTo_i16, expect: i16) {
			assert_eq!(input.pad_to(), expect);
		}
	}
}

pub mod gen_i32 {
	use super::*;

	impl<T> PadTo<i32> for T where T: PadTo_i32 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to(&self) -> i32 {
			return self.pad_to_i32();
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i32() {
			inner(127_i8, 127);
			inner(-128_i8, -128);

			inner(32767_i16, 32767);
			inner(-32768_i16, -32768);

			inner(2147483647_i32, 2147483647);
			inner(-2147483648_i32, -2147483648);

			#[cfg(target_pointer_width = "32")]
			{
				inner(isize::MIN, isize::MIN as i32);
				inner(isize::MAX, isize::MAX as i32);
			}
			
			
			inner(0_u8, 0);
			inner(255_u8, 255);
			
			inner(0_u16, 0);
			inner(65535_u16, 65535);
		}
		
		fn inner(input: impl PadTo_i32, expect: i32) {
			assert_eq!(input.pad_to(), expect);
		}
	}
}

pub mod gen_i64 {
	use super::*;

	impl<T> PadTo<i64> for T where T: PadTo_i64 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to(&self) -> i64 {
			return self.pad_to_i64();
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i64() {
			inner(127_i8, 127);
			inner(-128_i8, -128);

			inner(32767_i16, 32767);
			inner(-32768_i16, -32768);

			inner(2147483647_i32, 2147483647);
			inner(-2147483648_i32, -2147483648);

			inner(9223372036854775807_i64, 9223372036854775807);
			inner(-9223372036854775808_i64, -9223372036854775808);
			
			inner(isize::MIN, isize::MIN as i64);
			inner(isize::MAX, isize::MAX as i64);
			
			inner(0_u8, 0);
			inner(255_u8, 255);
			
			inner(0_u16, 0);
			inner(65535_u16, 65535);
			
			inner(0_u32, 0);
			inner(4294967295_u32, 4294967295);

			#[cfg(target_pointer_width = "32")]
			{
				inner(0_usize, 0);
				inner(usize::MAX, usize::MAX as i64);
			}
		}
		
		fn inner(input: impl PadTo_i64, expect: i64) {
			assert_eq!(input.pad_to(), expect);
		}
	}
}

pub mod gen_i128 {
	use super::*;

	impl<T> PadTo<i128> for T where T: PadTo_i128 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to(&self) -> i128 {
			return self.pad_to_i128();
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i128() {
			inner(127_i8, 127);
			inner(-128_i8, -128);

			inner(32767_i16, 32767);
			inner(-32768_i16, -32768);

			inner(2147483647_i32, 2147483647);
			inner(-2147483648_i32, -2147483648);

			inner(9223372036854775807_i64, 9223372036854775807);
			inner(-9223372036854775808_i64, -9223372036854775808);
			
			inner(isize::MIN, isize::MIN as i128);
			inner(isize::MAX, isize::MAX as i128);

			inner(170141183460469231731687303715884105727_i128, 170141183460469231731687303715884105727);
			inner(-170141183460469231731687303715884105728_i128, -170141183460469231731687303715884105728);
			
			inner(0_u8, 0);
			inner(255_u8, 255);
			
			inner(0_u16, 0);
			inner(65535_u16, 65535);
			
			inner(0_u32, 0);
			inner(4294967295_u32, 4294967295);
			
			inner(0_u64, 0);
			inner(18446744073709551615_u64, 18446744073709551615);
			
			inner(0_usize, 0);
			inner(usize::MAX, usize::MAX as i128);
		}
		
		fn inner(input: impl PadTo_i128, expect: i128) {
			assert_eq!(input.pad_to(), expect);
		}
	}
}

pub mod gen_isize {
	use super::*;

	impl<T> PadTo<isize> for T where T: PadTo_isize {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to(&self) -> isize {
			return self.pad_to_isize();
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_isize() {
			inner(127_i8, 127);
			inner(-128_i8, -128);

			inner(32767_i16, 32767);
			inner(-32768_i16, -32768);

			inner(2147483647_i32, 2147483647);
			inner(-2147483648_i32, -2147483648);

			#[cfg(target_pointer_width = "64")]
			{
				inner(9223372036854775807_i64, 9223372036854775807);
				inner(-9223372036854775808_i64, -9223372036854775808);
			}
			
			inner(isize::MIN, isize::MIN);
			inner(isize::MAX, isize::MAX);
			
			inner(0_u8, 0);
			inner(255_u8, 255);
			
			inner(0_u16, 0);
			inner(65535_u16, 65535);

			#[cfg(target_pointer_width = "64")]
			{
				inner(0_u32, 0);
				inner(4294967295_u32, 4294967295);
			}
		}
		
		fn inner(input: impl PadTo_isize, expect: isize) {
			assert_eq!(input.pad_to(), expect);
		}
	}
}

pub mod gen_u16 {
	use super::*;

	impl<T> PadTo<u16> for T where T: PadTo_u16 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to(&self) -> u16 {
			return self.pad_to_u16();
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_u16() {
			inner(0_u8, 0);
			inner(255_u8, 255);
			
			inner(0_u16, 0);
			inner(65535_u16, 65535);
		}
		
		fn inner(input: impl PadTo_u16, expect: u16) {
			assert_eq!(input.pad_to(), expect);
		}
	}
}

pub mod gen_u32 {
	use super::*;

	impl<T> PadTo<u32> for T where T: PadTo_u32 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to(&self) -> u32 {
			return self.pad_to_u32();
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_u32() {
			inner(0_u8, 0);
			inner(255_u8, 255);
			
			inner(0_u16, 0);
			inner(65535_u16, 65535);
			
			inner(0_u32, 0);
			inner(4294967295_u32, 4294967295);
			
			#[cfg(target_pointer_width = "32")]
			{
				inner(0_usize, 0);
				inner(usize::MAX, usize::MAX as u32);
			}
		}
		
		fn inner(input: impl PadTo_u32, expect: u32) {
			assert_eq!(input.pad_to(), expect);
		}
	}
}

pub mod gen_u64 {
	use super::*;

	impl<T> PadTo<u64> for T where T: PadTo_u64 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to(&self) -> u64 {
			return self.pad_to_u64();
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_u64() {
			inner(0_u8, 0);
			inner(255_u8, 255);
			
			inner(0_u16, 0);
			inner(65535_u16, 65535);
			
			inner(0_u32, 0);
			inner(4294967295_u32, 4294967295);
			
			inner(0_u64, 0);
			inner(18446744073709551615_u64, 18446744073709551615);
			
			inner(0_usize, 0);
			inner(usize::MAX, usize::MAX as u64);
		}
		
		fn inner(input: impl PadTo_u64, expect: u64) {
			assert_eq!(input.pad_to(), expect);
		}
	}
}

pub mod gen_u128 {
	use super::*;

	impl<T> PadTo<u128> for T where T: PadTo_u128 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to(&self) -> u128 {
			return self.pad_to_u128();
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_u128() {
			inner(0_u8, 0);
			inner(255_u8, 255);
			
			inner(0_u16, 0);
			inner(65535_u16, 65535);
			
			inner(0_u32, 0);
			inner(4294967295_u32, 4294967295);
			
			inner(0_u64, 0);
			inner(18446744073709551615_u64, 18446744073709551615);
			
			inner(0_u128, 0);
			inner(340282366920938463463374607431768211455_u128, 340282366920938463463374607431768211455);
			
			inner(0_usize, 0);
			inner(usize::MAX, usize::MAX as u128);
		}
		
		fn inner(input: impl PadTo_u128, expect: u128) {
			assert_eq!(input.pad_to(), expect);
		}
	}
}

pub mod gen_usize {
	use super::*;

	impl<T> PadTo<usize> for T where T: PadTo_usize {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to(&self) -> usize {
			return self.pad_to_usize();
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_usize() {
			inner(0_u8, 0);
			inner(255_u8, 255);
			
			inner(0_u16, 0);
			inner(65535_u16, 65535);
			
			inner(0_u32, 0);
			inner(4294967295_u32, 4294967295);
			
			#[cfg(target_pointer_width = "64")]
			{
				inner(0_u64, 0);
				inner(18446744073709551615_u64, 18446744073709551615);
			}
			
			inner(0_usize, 0);
			inner(usize::MAX, usize::MAX);
		}
		
		fn inner(input: impl PadTo_usize, expect: usize) {
			assert_eq!(input.pad_to(), expect);
		}
	}
}