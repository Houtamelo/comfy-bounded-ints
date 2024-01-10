pub trait PadTo_isize {
	fn pad_to_isize(&self) -> isize;
}

pub mod i8 {
	use super::*;

	impl PadTo_isize for i8 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_isize(&self) -> isize {
			return *self as isize;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_isize() {
			assert_eq!({ -1_i8 }.pad_to_isize(), -1_isize);
			assert_eq!(0_i8.pad_to_isize(), 0_isize);
			assert_eq!(127_i8.pad_to_isize(), 127_isize);
			assert_eq!({ -128_i8 }.pad_to_isize(), -128_isize);
		}
	}
}

pub mod i16 {
	use super::*;

	impl PadTo_isize for i16 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_isize(&self) -> isize {
			return *self as isize;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_isize() {
			assert_eq!({ -1_i16 }.pad_to_isize(), -1_isize);
			assert_eq!(0_i16.pad_to_isize(), 0_isize);
			assert_eq!(32767_i16.pad_to_isize(), 32767_isize);
			assert_eq!({ -32768_i16 }.pad_to_isize(), -32768_isize);
		}
	}
}

pub mod i32 {
	use super::*;

	impl PadTo_isize for i32 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_isize(&self) -> isize {
			return *self as isize;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_isize() {
			assert_eq!({ -1_i32 }.pad_to_isize(), -1_isize);
			assert_eq!(0_i32.pad_to_isize(), 0_isize);
			assert_eq!(2147483647_i32.pad_to_isize(), 2147483647_isize);
			assert_eq!({ -2147483648_i32 }.pad_to_isize(), -2147483648_isize);
		}
	}
}

#[cfg(target_pointer_width = "64")]
pub mod i64 {
	use super::*;

	impl PadTo_isize for i64 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_isize(&self) -> isize {
			return *self as isize;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_isize() {
			assert_eq!({ -1_i64 }.pad_to_isize(), -1_isize);
			assert_eq!(0_i64.pad_to_isize(), 0_isize);
			assert_eq!(2147483647_i64.pad_to_isize(), 2147483647_isize);
			assert_eq!({ -2147483648_i64 }.pad_to_isize(), -2147483648_isize);
		}
	}
}

pub mod isize {
	use super::*;

	impl PadTo_isize for isize {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_isize(&self) -> isize { *self }
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_isize() {
			assert_eq!({ -1_isize }.pad_to_isize(), -1_isize);
			assert_eq!(0_isize.pad_to_isize(), 0_isize);
			assert_eq!(isize::MAX.pad_to_isize(), isize::MAX);
			assert_eq!(isize::MIN.pad_to_isize(), isize::MIN);
		}
	}
}

pub mod u8 {
	use super::*;

	impl PadTo_isize for u8 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_isize(&self) -> isize {
			return *self as isize;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_isize() {
			assert_eq!(0_u8.pad_to_isize(), 0_isize);
			assert_eq!(255_u8.pad_to_isize(), 255_isize);
		}
	}
}

pub mod u16 {
	use super::*;

	impl PadTo_isize for u16 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_isize(&self) -> isize {
			return *self as isize;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_isize() {
			assert_eq!(0_u16.pad_to_isize(), 0_isize);
			assert_eq!(65535_u16.pad_to_isize(), 65535_isize);
		}
	}
}

#[cfg(target_pointer_width = "64")]
pub mod u32 {
	use super::*;

	impl PadTo_isize for u32 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_isize(&self) -> isize {
			return *self as isize;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_isize() {
			assert_eq!(0_u32.pad_to_isize(), 0_isize);
			assert_eq!(4294967295_u32.pad_to_isize(), 4294967295_isize);
		}
	}
}
