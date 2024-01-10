pub trait PadTo_i64 {
	fn pad_to_i64(&self) -> i64;
}

pub mod i8 {
	use super::*;

	impl PadTo_i64 for i8 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_i64(&self) -> i64 {
			return *self as i64;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i64() {
			assert_eq!({ -1_i8 }.pad_to_i64(), -1_i64);
			assert_eq!(0_i8.pad_to_i64(), 0_i64);
			assert_eq!(127_i8.pad_to_i64(), 127_i64);
			assert_eq!({-128_i8}.pad_to_i64(), -128_i64);
		}
	}
}

pub mod i16 {
	use super::*;

	impl PadTo_i64 for i16 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_i64(&self) -> i64 {
			return *self as i64;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i64() {
			assert_eq!({ -1_i16 }.pad_to_i64(), -1_i64);
			assert_eq!(0_i16.pad_to_i64(), 0_i64);
			assert_eq!(32767_i16.pad_to_i64(), 32767_i64);
			assert_eq!({-32768_i16}.pad_to_i64(), -32768_i64);
		}
	}
}

pub mod i32 {
	use super::*;

	impl PadTo_i64 for i32 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_i64(&self) -> i64 {
			return *self as i64;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i64() {
			assert_eq!({ -1_i32 }.pad_to_i64(), -1_i64);
			assert_eq!(0_i32.pad_to_i64(), 0_i64);
			assert_eq!(2147483647_i32.pad_to_i64(), 2147483647_i64);
			assert_eq!({-2147483648_i32}.pad_to_i64(), -2147483648_i64);
		}
	}
}

pub mod i64 {
	use super::*;

	impl PadTo_i64 for i64 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_i64(&self) -> i64 { *self }
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i64() {
			assert_eq!({ -1_i64 }.pad_to_i64(), -1_i64);
			assert_eq!(0_i64.pad_to_i64(), 0_i64);
			assert_eq!(9223372036854775807_i64.pad_to_i64(), 9223372036854775807_i64);
			assert_eq!({-9223372036854775808_i64}.pad_to_i64(), -9223372036854775808_i64);
		}
	}
}

pub mod isize {
	use super::*;

	impl PadTo_i64 for isize {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_i64(&self) -> i64 {
			return *self as i64;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i64() {
			assert_eq!({ -1_isize }.pad_to_i64(), -1_i64);
			assert_eq!(0_isize.pad_to_i64(), 0_i64);
			assert_eq!(isize::MAX.pad_to_i64(), isize::MAX as i64);
			assert_eq!(isize::MIN.pad_to_i64(), isize::MIN as i64);
		}
	}
}

pub mod u8 {
	use super::*;

	impl PadTo_i64 for u8 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_i64(&self) -> i64 {
			return *self as i64;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i64() {
			assert_eq!(0_u8.pad_to_i64(), 0_i64);
			assert_eq!(255_u8.pad_to_i64(), 255_i64);
		}
	}
}

pub mod u16 {
	use super::*;

	impl PadTo_i64 for u16 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_i64(&self) -> i64 {
			return *self as i64;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i64() {
			assert_eq!(0_u16.pad_to_i64(), 0_i64);
			assert_eq!(65535_u16.pad_to_i64(), 65535_i64);
		}
	}
}

pub mod u32 {
	use super::*;

	impl PadTo_i64 for u32 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_i64(&self) -> i64 {
			return *self as i64;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i64() {
			assert_eq!(0_u32.pad_to_i64(), 0_i64);
			assert_eq!(4294967295_u32.pad_to_i64(), 4294967295_i64);
		}
	}
}

#[cfg(target_pointer_width = "32")]
pub mod usize {
	use super::*;

	impl PadTo_i64 for usize {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_i64(&self) -> i64 {
			return *self as i64;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i64() {
			assert_eq!(0_usize.pad_to_i64(), 0_i64);
			assert_eq!(usize::MAX.pad_to_i64(), usize::MAX as i64);
		}
	}
}
