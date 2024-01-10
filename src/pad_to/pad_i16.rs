pub trait PadTo_i16 {
	fn pad_to_i16(&self) -> i16;
}

pub mod i8 {
	use super::*;

	impl PadTo_i16 for i8 {
		#[inline(always)] #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_i16(&self) -> i16 {
			return *self as i16;
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i16() {
			assert_eq!({ -1_i8 }.pad_to_i16(), -1_i16);
			assert_eq!(0_i8.pad_to_i16(), 0_i16);
			assert_eq!(127_i8.pad_to_i16(), 127_i16);
			assert_eq!({-128_i8}.pad_to_i16(), -128_i16);
		}
	}
}

pub mod i16 {
	use super::*;

	impl PadTo_i16 for i16 {
		#[inline(always)] #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_i16(&self) -> i16 { *self }
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_i16() {
			assert_eq!({ -1_i16 }.pad_to_i16(), -1_i16);
			assert_eq!(0_i16.pad_to_i16(), 0_i16);
			assert_eq!(32767_i16.pad_to_i16(), 32767_i16);
			assert_eq!({-32768_i16}.pad_to_i16(), -32768_i16);
		}
	}
}

pub mod u8 {
	use super::*;

	impl PadTo_i16 for u8 {
		#[inline(always)] #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_i16(&self) -> i16 {
			return *self as i16;
		}
	}

	#[cfg(test)]
	mod tests_u8 {
		use super::*;

		#[test]
		fn test_pad_to_i16() {
			assert_eq!(0_u8.pad_to_i16(), 0_i16);
			assert_eq!(255_u8.pad_to_i16(), 255_i16);
		}
	}
}