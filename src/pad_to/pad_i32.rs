pub trait PadTo_i32 {
	fn pad_to_i32(&self) -> i32;
}

pub mod i8 {
	use super::*;

	impl PadTo_i32 for i8 {
		#[inline(always)] #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_i32(&self) -> i32 {
			return *self as i32;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i32() {
			assert_eq!({ -1_i8 }.pad_to_i32(), -1_i32);
			assert_eq!(0_i8.pad_to_i32(), 0_i32);
			assert_eq!(127_i8.pad_to_i32(), 127_i32);
			assert_eq!({-128_i8}.pad_to_i32(), -128_i32);
		}
	}
}

pub mod i16 {
	use super::*;

	impl PadTo_i32 for i16 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_i32(&self) -> i32 {
			return *self as i32;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i32() {
			assert_eq!({ -1_i16 }.pad_to_i32(), -1_i32);
			assert_eq!(0_i16.pad_to_i32(), 0_i32);
			assert_eq!(32767_i16.pad_to_i32(), 32767_i32);
			assert_eq!({-32768_i16}.pad_to_i32(), -32768_i32);
		}
	}
}

pub mod i32 {
	use super::*;

	impl PadTo_i32 for i32 {
		#[inline(always)] #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_i32(&self) -> i32 { *self }
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i32() {
			assert_eq!({ -1_i32 }.pad_to_i32(), -1_i32);
			assert_eq!(0_i32.pad_to_i32(), 0_i32);
			assert_eq!(2147483647_i32.pad_to_i32(), 2147483647_i32);
			assert_eq!({-2147483648_i32}.pad_to_i32(), -2147483648_i32);
		}
	}
}

#[cfg(target_pointer_width = "32")]
pub mod isize {
	use super::*;

	impl PadTo_i32 for isize {
		#[inline(always)] #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_i32(&self) -> i32 {
			return *self as i32;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i32() {
			assert_eq!({ -1_isize }.pad_to_i32(), -1_i32);
			assert_eq!(0_isize.pad_to_i32(), 0_i32);
			assert_eq!(isize::MAX.pad_to_i32(), isize::MAX as i32);
			assert_eq!(isize::MIN.pad_to_i32(), isize::MIN as i32);
		}
	}
}

pub mod u8 {
	use super::*;

	impl PadTo_i32 for u8 {
		#[inline(always)] #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_i32(&self) -> i32 {
			return *self as i32;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i32() {
			assert_eq!(0_u8.pad_to_i32(), 0_i32);
			assert_eq!(255_u8.pad_to_i32(), 255_i32);
		}
	}
}

pub mod u16 {
	use super::*;

	impl PadTo_i32 for u16 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_i32(&self) -> i32 {
			return *self as i32;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i32() {
			assert_eq!(0_u16.pad_to_i32(), 0_i32);
			assert_eq!(65535_u16.pad_to_i32(), 65535_i32);
		}
	}
}