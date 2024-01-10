pub trait PadTo_i128 {
	fn pad_to_i128(&self) -> i128;
}

pub mod i8 {
	use super::*;

	impl PadTo_i128 for i8 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_i128(&self) -> i128 {
			return *self as i128;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i128() {
			assert_eq!({ -1_i8 }.pad_to_i128(), -1_i128);
			assert_eq!(0_i8.pad_to_i128(), 0_i128);
			assert_eq!(127_i8.pad_to_i128(), 127_i128);
			assert_eq!({-128_i8}.pad_to_i128(), -128_i128);
		}
	}
}

pub mod i16 {
	use super::*;

	impl PadTo_i128 for i16 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_i128(&self) -> i128 {
			return *self as i128;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i128() {
			assert_eq!({ -1_i16 }.pad_to_i128(), -1_i128);
			assert_eq!(0_i16.pad_to_i128(), 0_i128);
			assert_eq!(32767_i16.pad_to_i128(), 32767_i128);
			assert_eq!({-32768_i16}.pad_to_i128(), -32768_i128);
		}
	}
}

pub mod i32 {
	use super::*;

	impl PadTo_i128 for i32 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_i128(&self) -> i128 {
			return *self as i128;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i128() {
			assert_eq!({ -1_i32 }.pad_to_i128(), -1_i128);
			assert_eq!(0_i32.pad_to_i128(), 0_i128);
			assert_eq!(32767_i32.pad_to_i128(), 32767_i128);
			assert_eq!({-32768_i32}.pad_to_i128(), -32768_i128);
		}
	}
}

pub mod i64 {
	use super::*;

	impl PadTo_i128 for i64 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_i128(&self) -> i128 {
			return *self as i128;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i128() {
			assert_eq!({ -1_i64 }.pad_to_i128(), -1_i128);
			assert_eq!(0_i64.pad_to_i128(), 0_i128);
			assert_eq!(2147483647_i64.pad_to_i128(), 2147483647_i128);
			assert_eq!({-2147483648_i64}.pad_to_i128(), -2147483648_i128);
		}
	}
}

pub mod i128 {
	use super::*;

	impl PadTo_i128 for i128 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_i128(&self) -> i128 { *self }
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i128() {
			assert_eq!({ -1_i128 }.pad_to_i128(), -1_i128);
			assert_eq!(0_i128.pad_to_i128(), 0_i128);
			assert_eq!(9223372036854775807_i128.pad_to_i128(), 9223372036854775807_i128);
			assert_eq!({-9223372036854775808_i128}.pad_to_i128(), -9223372036854775808_i128);
		}
	}
}

pub mod isize {
	use super::*;

	impl PadTo_i128 for isize {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_i128(&self) -> i128 {
			return *self as i128;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i128() {
			assert_eq!({ -1_isize }.pad_to_i128(), -1_i128);
			assert_eq!(1_isize.pad_to_i128(), 1_i128);
			assert_eq!(isize::MAX.pad_to_i128(), isize::MAX as i128);
			assert_eq!(isize::MIN.pad_to_i128(), isize::MIN as i128);
		}
	}
}

pub mod u8 {
	use super::*;

	impl PadTo_i128 for u8 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_i128(&self) -> i128 {
			return *self as i128;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i128() {
			assert_eq!(0_u8.pad_to_i128(), 0_i128);
			assert_eq!(255_u8.pad_to_i128(), 255_i128);
		}
	}
}

pub mod u16 {
	use super::*;

	impl PadTo_i128 for u16 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_i128(&self) -> i128 {
			return *self as i128;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i128() {
			assert_eq!(0_u16.pad_to_i128(), 0_i128);
			assert_eq!(65535_u16.pad_to_i128(), 65535_i128);
		}
	}
}

pub mod u32 {
	use super::*;

	impl PadTo_i128 for u32 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_i128(&self) -> i128 {
			return *self as i128;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i128() {
			assert_eq!(0_u32.pad_to_i128(), 0_i128);
			assert_eq!(4294967295_u32.pad_to_i128(), 4294967295_i128);
		}
	}
}

pub mod u64 {
	use super::*;

	impl PadTo_i128 for u64 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_i128(&self) -> i128 {
			return *self as i128;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i128() {
			assert_eq!(0_u64.pad_to_i128(), 0_i128);
			assert_eq!(18446744073709551615_u64.pad_to_i128(), 18446744073709551615_i128);
		}
	}
}

pub mod usize {
	use super::*;

	impl PadTo_i128 for usize {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_i128(&self) -> i128 {
			return *self as i128;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i128() {
			assert_eq!(0_usize.pad_to_i128(), 0_i128);
			assert_eq!(usize::MAX.pad_to_i128(), usize::MAX as i128);
		}
	}
}
