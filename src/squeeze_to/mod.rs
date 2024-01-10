pub use squeeze_i8::SqueezeTo_i8;
pub use squeeze_i16::SqueezeTo_i16;
pub use squeeze_i32::SqueezeTo_i32;
pub use squeeze_i64::SqueezeTo_i64;
pub use squeeze_i128::SqueezeTo_i128;
pub use squeeze_isize::SqueezeTo_isize;

pub use squeeze_u8::SqueezeTo_u8;
pub use squeeze_u16::SqueezeTo_u16;
pub use squeeze_u32::SqueezeTo_u32;
pub use squeeze_u64::SqueezeTo_u64;
pub use squeeze_u128::SqueezeTo_u128;
pub use squeeze_usize::SqueezeTo_usize;

mod squeeze_i8;
mod squeeze_i16;
mod squeeze_i32;
mod squeeze_i64;
mod squeeze_i128;
mod squeeze_isize;

mod squeeze_u8;
mod squeeze_u16;
mod squeeze_u32;
mod squeeze_u64;
mod squeeze_u128;
mod squeeze_usize;

pub trait SqueezeTo<T> {
	fn squeeze_to(&self) -> T;
}

mod gen_i8 {
	use super::*;
	
	impl<T> SqueezeTo<i8> for T where T: SqueezeTo_i8 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to(&self) -> i8 {
			return self.squeeze_to_i8();
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_i8() {
			inner(i8::MAX, i8::MAX);
			inner(i8::MIN, i8::MIN);
			inner(5_i8, 5);
			inner(-5_i8, -5);

			inner(i16::MAX, i8::MAX);
			inner(i16::MIN, i8::MIN);
			inner(5_i16, 5);
			inner(-5_i16, -5);

			inner(i32::MAX, i8::MAX);
			inner(i32::MIN, i8::MIN);
			inner(5_i32, 5);
			inner(-5_i32, -5);

			inner(i64::MAX, i8::MAX);
			inner(i64::MIN, i8::MIN);
			inner(5_i64, 5);
			inner(-5_i64, -5);

			inner(i128::MAX, i8::MAX);
			inner(i128::MIN, i8::MIN);
			inner(5_i128, 5);
			inner(-5_i128, -5);

			inner(isize::MAX, i8::MAX);
			inner(isize::MIN, i8::MIN);
			inner(5_isize, 5);
			inner(-5_isize, -5);

			inner(u8::MAX, i8::MAX);
			inner(0_u8, 0);
			inner(5_u8, 5);

			inner(u16::MAX, i8::MAX);
			inner(0_u16, 0);
			inner(5_u16, 5);

			inner(u32::MAX, i8::MAX);
			inner(0_u32, 0);
			inner(5_u32, 5);

			inner(u64::MAX, i8::MAX);
			inner(0_u64, 0);
			inner(5_u64, 5);

			inner(u128::MAX, i8::MAX);
			inner(0_u128, 0);
			inner(5_u128, 5);

			inner(usize::MAX, i8::MAX);
			inner(0_usize, 0);
			inner(5_usize, 5);
		}

		fn inner(input: impl SqueezeTo_i8, expect: i8) {
			assert_eq!(input.squeeze_to(), expect);
		}
	}
}

mod gen_i16 {
	use super::*;
	impl<T> SqueezeTo<i16> for T where T: SqueezeTo_i16 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to(&self) -> i16 {
			return self.squeeze_to_i16();
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_i16() {
			inner(i8::MAX, i8::MAX as i16);
			inner(i8::MIN, i8::MIN as i16);
			inner(5_i8, 5);
			inner(-5_i8, -5);

			inner(i16::MAX, i16::MAX);
			inner(i16::MIN, i16::MIN);
			inner(5_i16, 5);
			inner(-5_i16, -5);

			inner(i32::MAX, i16::MAX);
			inner(i32::MIN, i16::MIN);
			inner(5_i32, 5);
			inner(-5_i32, -5);

			inner(i64::MAX, i16::MAX);
			inner(i64::MIN, i16::MIN);
			inner(5_i64, 5);
			inner(-5_i64, -5);

			inner(i128::MAX, i16::MAX);
			inner(i128::MIN, i16::MIN);
			inner(5_i128, 5);
			inner(-5_i128, -5);

			inner(isize::MAX, i16::MAX);
			inner(isize::MIN, i16::MIN);
			inner(5_isize, 5);
			inner(-5_isize, -5);

			inner(u8::MAX, u8::MAX as i16);
			inner(0_u8, 0);
			inner(5_u8, 5);

			inner(u16::MAX, i16::MAX);
			inner(0_u16, 0);
			inner(5_u16, 5);

			inner(u32::MAX, i16::MAX);
			inner(0_u32, 0);
			inner(5_u32, 5);

			inner(u64::MAX, i16::MAX);
			inner(0_u64, 0);
			inner(5_u64, 5);

			inner(u128::MAX, i16::MAX);
			inner(0_u128, 0);
			inner(5_u128, 5);

			inner(usize::MAX, i16::MAX);
			inner(0_usize, 0);
			inner(5_usize, 5);
		}

		fn inner(input: impl SqueezeTo_i16, expect: i16) {
			assert_eq!(input.squeeze_to(), expect);
		}
	}
}

mod gen_i32 {
	use super::*;
	impl<T> SqueezeTo<i32> for T where T: SqueezeTo_i32 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to(&self) -> i32 {
			return self.squeeze_to_i32();
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_i32() {
			inner(i8::MAX, i8::MAX as i32);
			inner(i8::MIN, i8::MIN as i32);
			inner(5_i8, 5);
			inner(-5_i8, -5);

			inner(i16::MAX, i16::MAX as i32);
			inner(i16::MIN, i16::MIN as i32);
			inner(5_i16, 5);
			inner(-5_i16, -5);

			inner(i32::MAX, i32::MAX);
			inner(i32::MIN, i32::MIN);
			inner(5_i32, 5);
			inner(-5_i32, -5);

			inner(i64::MAX, i32::MAX);
			inner(i64::MIN, i32::MIN);
			inner(5_i64, 5);
			inner(-5_i64, -5);

			inner(i128::MAX, i32::MAX);
			inner(i128::MIN, i32::MIN);
			inner(5_i128, 5);
			inner(-5_i128, -5);

			inner(isize::MAX, i32::MAX);
			inner(isize::MIN, i32::MIN);
			inner(5_isize, 5);
			inner(-5_isize, -5);

			inner(u8::MAX, u8::MAX as i32);
			inner(0_u8, 0);
			inner(5_u8, 5);

			inner(u16::MAX, u16::MAX as i32);
			inner(0_u16, 0);
			inner(5_u16, 5);

			inner(u32::MAX, i32::MAX);
			inner(0_u32, 0);
			inner(5_u32, 5);

			inner(u64::MAX, i32::MAX);
			inner(0_u64, 0);
			inner(5_u64, 5);

			inner(u128::MAX, i32::MAX);
			inner(0_u128, 0);
			inner(5_u128, 5);

			inner(usize::MAX, i32::MAX);
			inner(0_usize, 0);
			inner(5_usize, 5);
		}

		fn inner(input: impl SqueezeTo_i32, expect: i32) {
			assert_eq!(input.squeeze_to(), expect);
		}
	}
}

mod gen_i64 {
	use super::*;
	impl<T> SqueezeTo<i64> for T where T: SqueezeTo_i64 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to(&self) -> i64 {
			return self.squeeze_to_i64();
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_i64() {
			inner(i8::MAX, i8::MAX as i64);
			inner(i8::MIN, i8::MIN as i64);
			inner(5_i8, 5);
			inner(-5_i8, -5);
			
			inner(i16::MAX, i16::MAX as i64);
			inner(i16::MIN, i16::MIN as i64);
			inner(5_i16, 5);
			inner(-5_i16, -5);

			inner(i32::MAX, i32::MAX as i64);
			inner(i32::MIN, i32::MIN as i64);
			inner(5_i32, 5);
			inner(-5_i32, -5);

			inner(i64::MAX, i64::MAX);
			inner(i64::MIN, i64::MIN);
			inner(5_i64, 5);
			inner(-5_i64, -5);

			inner(i128::MAX, i64::MAX);
			inner(i128::MIN, i64::MIN);
			inner(5_i128, 5);
			inner(-5_i128, -5);

			#[cfg(target_pointer_width = "32")]
			{
				inner(isize::MAX, isize::MAX as i64);
				inner(isize::MIN, isize::MIN as i64);
				inner(5_isize, 5);
				inner(-5_isize, -5);
			}

			#[cfg(target_pointer_width = "64")]
			{
				inner(isize::MAX, i64::MAX);
				inner(isize::MIN, i64::MIN);
				inner(5_isize, 5);
				inner(-5_isize, -5);
			}

			inner(u8::MAX, u8::MAX as i64);
			inner(0_u8, 0);
			inner(5_u8, 5);

			inner(u16::MAX, u16::MAX as i64);
			inner(0_u16, 0);
			inner(5_u16, 5);

			inner(u32::MAX, u32::MAX as i64);
			inner(0_u32, 0);
			inner(5_u32, 5);

			inner(u64::MAX, i64::MAX);
			inner(0_u64, 0);
			inner(5_u64, 5);

			inner(u128::MAX, i64::MAX);
			inner(0_u128, 0);
			inner(5_u128, 5);

			#[cfg(target_pointer_width = "32")]
			{
				inner(usize::MAX, usize::MAX as i64);
				inner(0_usize, 0);
				inner(5_usize, 5);
			}

			#[cfg(target_pointer_width = "64")]
			{
				inner(usize::MAX, i64::MAX);
				inner(0_usize, 0);
				inner(5_usize, 5);
			}
		}

		fn inner(input: impl SqueezeTo_i64, expect: i64) {
			assert_eq!(input.squeeze_to(), expect);
		}
	}
}

mod gen_i128 {
	use super::*;
	impl<T> SqueezeTo<i128> for T where T: SqueezeTo_i128 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to(&self) -> i128 {
			return self.squeeze_to_i128();
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_i128() {
			inner(i8::MAX, i8::MAX as i128);
			inner(i8::MIN, i8::MIN as i128);
			inner(5_i8, 5);
			inner(-5_i8, -5);
			
			inner(i16::MAX, i16::MAX as i128);
			inner(i16::MIN, i16::MIN as i128);
			inner(5_i16, 5);
			inner(-5_i16, -5);

			inner(i32::MAX, i32::MAX as i128);
			inner(i32::MIN, i32::MIN as i128);
			inner(5_i32, 5);
			inner(-5_i32, -5);

			inner(i64::MAX, i64::MAX as i128);
			inner(i64::MIN, i64::MIN as i128);
			inner(5_i64, 5);
			inner(-5_i64, -5);

			inner(i128::MAX, i128::MAX);
			inner(i128::MIN, i128::MIN);
			inner(5_i128, 5);
			inner(-5_i128, -5);
			
			inner(isize::MAX, isize::MAX as i128);
			inner(isize::MIN, isize::MIN as i128);
			inner(5_isize, 5);
			inner(-5_isize, -5);

			inner(u8::MAX, u8::MAX as i128);
			inner(0_u8, 0);
			inner(5_u8, 5);

			inner(u16::MAX, u16::MAX as i128);
			inner(0_u16, 0);
			inner(5_u16, 5);

			inner(u32::MAX, u32::MAX as i128);
			inner(0_u32, 0);
			inner(5_u32, 5);

			inner(u64::MAX, u64::MAX as i128);
			inner(0_u64, 0);
			inner(5_u64, 5);

			inner(u128::MAX, i128::MAX);
			inner(0_u128, 0);
			inner(5_u128, 5);

			inner(usize::MAX, usize::MAX as i128);
			inner(0_usize, 0);
			inner(5_usize, 5);
		}

		fn inner(input: impl SqueezeTo_i128, expect: i128) {
			assert_eq!(input.squeeze_to(), expect);
		}
	}
}

mod gen_isize {
	use super::*;
	impl<T> SqueezeTo<isize> for T where T: SqueezeTo_isize {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to(&self) -> isize {
			return self.squeeze_to_isize();
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_isize() {
			inner(i8::MAX, i8::MAX as isize);
			inner(i8::MIN, i8::MIN as isize);
			inner(5_i8, 5);
			inner(-5_i8, -5);

			inner(i16::MAX, i16::MAX as isize);
			inner(i16::MIN, i16::MIN as isize);
			inner(5_i16, 5);
			inner(-5_i16, -5);

			inner(i32::MAX, i32::MAX as isize);
			inner(i32::MIN, i32::MIN as isize);
			inner(5_i32, 5);
			inner(-5_i32, -5);

			#[cfg(target_pointer_width = "32")]
			{
				inner(i64::MAX, isize::MAX);
				inner(i64::MIN, isize::MIN);
				inner(5_i64, 5);
				inner(-5_i64, -5);
			}

			#[cfg(target_pointer_width = "64")]
			{
				inner(i64::MAX, i64::MAX as isize);
				inner(i64::MIN, i64::MIN as isize);
				inner(5_i64, 5);
				inner(-5_i64, -5);
			}

			inner(i128::MAX, isize::MAX);
			inner(i128::MIN, isize::MIN);
			inner(5_i128, 5);
			inner(-5_i128, -5);

			inner(isize::MAX, isize::MAX);
			inner(isize::MIN, isize::MIN);
			inner(5_isize, 5);
			inner(-5_isize, -5);

			inner(u8::MAX, u8::MAX as isize);
			inner(0_u8, 0);
			inner(5_u8, 5);

			inner(u16::MAX, u16::MAX as isize);
			inner(0_u16, 0);
			inner(5_u16, 5);

			#[cfg(target_pointer_width = "32")]
			{
				inner(u32::MAX, isize::MAX);
				inner(0_u32, 0);
				inner(5_u32, 5);
			}

			#[cfg(target_pointer_width = "64")]
			{
				inner(u32::MAX, u32::MAX as isize);
				inner(0_u32, 0);
				inner(5_u32, 5);
			}

			inner(u64::MAX, isize::MAX);
			inner(0_u64, 0);
			inner(5_u64, 5);

			inner(u128::MAX, isize::MAX);
			inner(0_u128, 0);
			inner(5_u128, 5);

			inner(usize::MAX, isize::MAX);
			inner(0_usize, 0);
			inner(5_usize, 5);
		}
		
		fn inner(input: impl SqueezeTo_isize, expect: isize) {
			assert_eq!(input.squeeze_to(), expect);
		}
	}
}

mod gen_u8 {
	use super::*;
	impl<T> SqueezeTo<u8> for T where T: SqueezeTo_u8 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to(&self) -> u8 {
			return self.squeeze_to_u8();
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_u8() {
			inner(i8::MAX, i8::MAX as u8);
			inner(i8::MIN, 0);
			inner(5_i8, 5);
			inner(-5_i8, 0);

			inner(i16::MAX, u8::MAX);
			inner(i16::MIN, 0);
			inner(5_i16, 5);
			inner(-5_i16, 0);

			inner(i32::MAX, u8::MAX);
			inner(i32::MIN, 0);
			inner(5_i32, 5);
			inner(-5_i32, 0);

			inner(i64::MAX, u8::MAX);
			inner(i64::MIN, 0);
			inner(5_i64, 5);
			inner(-5_i64, 0);

			inner(i128::MAX, u8::MAX);
			inner(i128::MIN, 0);
			inner(5_i128, 5);
			inner(-5_i128, 0);

			inner(isize::MAX, u8::MAX);
			inner(isize::MIN, 0);
			inner(5_isize, 5);
			inner(-5_isize, 0);

			inner(u8::MAX, u8::MAX);
			inner(0_u8, 0);
			inner(5_u8, 5);

			inner(u16::MAX, u8::MAX);
			inner(0_u16, 0);
			inner(5_u16, 5);

			inner(u32::MAX, u8::MAX);
			inner(0_u32, 0);
			inner(5_u32, 5);

			inner(u64::MAX, u8::MAX);
			inner(0_u64, 0);
			inner(5_u64, 5);

			inner(u128::MAX, u8::MAX);
			inner(0_u128, 0);
			inner(5_u128, 5);

			inner(usize::MAX, u8::MAX);
			inner(0_usize, 0);
			inner(5_usize, 5);
		}

		fn inner(input: impl SqueezeTo_u8, expect: u8) {
			assert_eq!(input.squeeze_to(), expect);
		}
	}
}

mod gen_u16 {
	use super::*;
	impl<T> SqueezeTo<u16> for T where T: SqueezeTo_u16 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to(&self) -> u16 {
			return self.squeeze_to_u16();
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_u16() {
			inner(i8::MAX, i8::MAX as u16);
			inner(i8::MIN, 0);
			inner(5_i8, 5);
			inner(-5_i8, 0);

			inner(i16::MAX, i16::MAX as u16);
			inner(i16::MIN, 0);
			inner(5_i16, 5);
			inner(-5_i16, 0);

			inner(i32::MAX, u16::MAX);
			inner(i32::MIN, 0);
			inner(5_i32, 5);
			inner(-5_i32, 0);

			inner(i64::MAX, u16::MAX);
			inner(i64::MIN, 0);
			inner(5_i64, 5);
			inner(-5_i64, 0);

			inner(i128::MAX, u16::MAX);
			inner(i128::MIN, 0);
			inner(5_i128, 5);
			inner(-5_i128, 0);

			inner(isize::MAX, u16::MAX);
			inner(isize::MIN, 0);
			inner(5_isize, 5);
			inner(-5_isize, 0);

			inner(u8::MAX, u8::MAX as u16);
			inner(0_u8, 0);
			inner(5_u8, 5);

			inner(u16::MAX, u16::MAX);
			inner(0_u16, 0);
			inner(5_u16, 5);

			inner(u32::MAX, u16::MAX);
			inner(0_u32, 0);
			inner(5_u32, 5);

			inner(u64::MAX, u16::MAX);
			inner(0_u64, 0);
			inner(5_u64, 5);

			inner(u128::MAX, u16::MAX);
			inner(0_u128, 0);
			inner(5_u128, 5);

			inner(usize::MAX, u16::MAX);
			inner(0_usize, 0);
			inner(5_usize, 5);
		}

		fn inner(input: impl SqueezeTo_u16, expect: u16) {
			assert_eq!(input.squeeze_to(), expect);
		}
	}
}

mod gen_u32 {
	use super::*;
	impl<T> SqueezeTo<u32> for T where T: SqueezeTo_u32 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to(&self) -> u32 {
			return self.squeeze_to_u32();
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_u32() {
			inner(i8::MAX, i8::MAX as u32);
			inner(i8::MIN, 0);
			inner(5_i8, 5);
			inner(-5_i8, 0);

			inner(i16::MAX, i16::MAX as u32);
			inner(i16::MIN, 0);
			inner(5_i16, 5);
			inner(-5_i16, 0);

			inner(i32::MAX, i32::MAX as u32);
			inner(i32::MIN, 0);
			inner(5_i32, 5);
			inner(-5_i32, 0);

			inner(i64::MAX, u32::MAX);
			inner(i64::MIN, 0);
			inner(5_i64, 5);
			inner(-5_i64, 0);

			inner(i128::MAX, u32::MAX);
			inner(i128::MIN, 0);
			inner(5_i128, 5);
			inner(-5_i128, 0);
			
			#[cfg(target_pointer_width = "32")]
			{
				inner(isize::MAX, isize::MAX as u32);
				inner(isize::MIN, 0);
				inner(5_isize, 5);
				inner(-5_isize, 0);
			}

			#[cfg(target_pointer_width = "64")]
			{
				inner(isize::MAX, u32::MAX);
				inner(isize::MIN, 0);
				inner(5_isize, 5);
				inner(-5_isize, 0);
			}

			inner(u8::MAX, u8::MAX as u32);
			inner(0_u8, 0);
			inner(5_u8, 5);

			inner(u16::MAX, u16::MAX as u32);
			inner(0_u16, 0);
			inner(5_u16, 5);

			inner(u32::MAX, u32::MAX);
			inner(0_u32, 0);
			inner(5_u32, 5);

			inner(u64::MAX, u32::MAX);
			inner(0_u64, 0);
			inner(5_u64, 5);

			inner(u128::MAX, u32::MAX);
			inner(0_u128, 0);
			inner(5_u128, 5);

			inner(usize::MAX, u32::MAX);
			inner(0_usize, 0);
			inner(5_usize, 5);
		}

		fn inner(input: impl SqueezeTo_u32, expect: u32) {
			assert_eq!(input.squeeze_to(), expect);
		}
	}
}

mod gen_u64 {
	use super::*;
	impl<T> SqueezeTo<u64> for T where T: SqueezeTo_u64 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to(&self) -> u64 {
			return self.squeeze_to_u64();
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_u64() {
			inner(i8::MAX, i8::MAX as u64);
			inner(i8::MIN, 0);
			inner(5_i8, 5);
			inner(-5_i8, 0);

			inner(i16::MAX, i16::MAX as u64);
			inner(i16::MIN, 0);
			inner(5_i16, 5);
			inner(-5_i16, 0);

			inner(i32::MAX, i32::MAX as u64);
			inner(i32::MIN, 0);
			inner(5_i32, 5);
			inner(-5_i32, 0);

			inner(i64::MAX, i64::MAX as u64);
			inner(i64::MIN, 0);
			inner(5_i64, 5);
			inner(-5_i64, 0);

			inner(i128::MAX, u64::MAX);
			inner(i128::MIN, 0);
			inner(5_i128, 5);
			inner(-5_i128, 0);

			inner(isize::MAX, isize::MAX as u64);
			inner(isize::MIN, 0);
			inner(5_isize, 5);
			inner(-5_isize, 0);

			inner(u8::MAX, u8::MAX as u64);
			inner(0_u8, 0);
			inner(5_u8, 5);

			inner(u16::MAX, u16::MAX as u64);
			inner(0_u16, 0);
			inner(5_u16, 5);

			inner(u32::MAX, u32::MAX as u64);
			inner(0_u32, 0);
			inner(5_u32, 5);

			inner(u64::MAX, u64::MAX);
			inner(0_u64, 0);
			inner(5_u64, 5);

			inner(u128::MAX, u64::MAX);
			inner(0_u128, 0);
			inner(5_u128, 5);

			inner(usize::MAX, usize::MAX as u64);
			inner(0_usize, 0);
			inner(5_usize, 5);
		}

		fn inner(input: impl SqueezeTo_u64, expect: u64) {
			assert_eq!(input.squeeze_to(), expect);
		}
	}
}

mod gen_u128 {
	use super::*;
	impl<T> SqueezeTo<u128> for T where T: SqueezeTo_u128 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to(&self) -> u128 {
			return self.squeeze_to_u128();
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_u64() {
			inner(i8::MAX, i8::MAX as u128);
			inner(i8::MIN, 0);
			inner(5_i8, 5);
			inner(-5_i8, 0);

			inner(i16::MAX, i16::MAX as u128);
			inner(i16::MIN, 0);
			inner(5_i16, 5);
			inner(-5_i16, 0);

			inner(i32::MAX, i32::MAX as u128);
			inner(i32::MIN, 0);
			inner(5_i32, 5);
			inner(-5_i32, 0);

			inner(i64::MAX, i64::MAX as u128);
			inner(i64::MIN, 0);
			inner(5_i64, 5);
			inner(-5_i64, 0);

			inner(i128::MAX, i128::MAX as u128);
			inner(i128::MIN, 0);
			inner(5_i128, 5);
			inner(-5_i128, 0);

			inner(isize::MAX, isize::MAX as u128);
			inner(isize::MIN, 0);
			inner(5_isize, 5);
			inner(-5_isize, 0);

			inner(u8::MAX, u8::MAX as u128);
			inner(0_u8, 0);
			inner(5_u8, 5);

			inner(u16::MAX, u16::MAX as u128);
			inner(0_u16, 0);
			inner(5_u16, 5);

			inner(u32::MAX, u32::MAX as u128);
			inner(0_u32, 0);
			inner(5_u32, 5);

			inner(u64::MAX, u64::MAX as u128);
			inner(0_u64, 0);
			inner(5_u64, 5);

			inner(u128::MAX, u128::MAX);
			inner(0_u128, 0);
			inner(5_u128, 5);

			inner(usize::MAX, usize::MAX as u128);
			inner(0_usize, 0);
			inner(5_usize, 5);
		}

		fn inner(input: impl SqueezeTo_u128, expect: u128) {
			assert_eq!(input.squeeze_to(), expect);
		}
	}
}

mod gen_usize {
	use super::*;
	impl<T> SqueezeTo<usize> for T where T: SqueezeTo_usize {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to(&self) -> usize {
			return self.squeeze_to_usize();
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_usize() {
			inner(i8::MAX, i8::MAX as usize);
			inner(i8::MIN, 0);
			inner(5_i8, 5);
			inner(-5_i8, 0);

			inner(i16::MAX, i16::MAX as usize);
			inner(i16::MIN, 0);
			inner(5_i16, 5);
			inner(-5_i16, 0);

			inner(i32::MAX, i32::MAX as usize);
			inner(i32::MIN, 0);
			inner(5_i32, 5);
			inner(-5_i32, 0);

			#[cfg(target_pointer_width = "32")]
			{
				inner(i64::MAX, usize::MAX);
				inner(i64::MIN, 0);
				inner(5_i64, 5);
				inner(-5_i64, 0);
			}

			#[cfg(target_pointer_width = "64")]
			{
				inner(i64::MAX, i64::MAX as usize);
				inner(i64::MIN, 0);
				inner(5_i64, 5);
				inner(-5_i64, 0);
			}

			inner(i128::MAX, usize::MAX);
			inner(i128::MIN, 0);
			inner(5_i128, 5);
			inner(-5_i128, 0);

			inner(isize::MAX, isize::MAX as usize);
			inner(isize::MIN, 0);
			inner(5_isize, 5);
			inner(-5_isize, 0);

			inner(u8::MAX, u8::MAX as usize);
			inner(0_u8, 0);
			inner(5_u8, 5);

			inner(u16::MAX, u16::MAX as usize);
			inner(0_u16, 0);
			inner(5_u16, 5);

			inner(u32::MAX, u32::MAX as usize);
			inner(0_u32, 0);
			inner(5_u32, 5);
			
			#[cfg(target_pointer_width = "32")]
			{
				inner(u64::MAX, usize::MAX);
				inner(0_u64, 0);
				inner(5_u64, 5);
			}
			
			#[cfg(target_pointer_width = "64")]
			{
				inner(u64::MAX, u64::MAX as usize);
				inner(0_u64, 0);
				inner(5_u64, 5);
			}
			
			inner(u128::MAX, usize::MAX);
			inner(0_u128, 0);
			inner(5_u128, 5);
			
			inner(usize::MAX, usize::MAX);
			inner(0_usize, 0);
			inner(5_usize, 5);
		}

		fn inner(input: impl SqueezeTo_usize, expect: usize) {
			assert_eq!(input.squeeze_to(), expect);
		}
	}
}