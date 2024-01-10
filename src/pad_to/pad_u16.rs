pub trait PadTo_u16 {
	fn pad_to_u16(&self) -> u16;
}

pub mod u8 {
	use super::*;

	impl PadTo_u16 for u8 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_u16(&self) -> u16 {
			return *self as u16;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_u16() {
			assert_eq!(0_u8.pad_to_u16(), 0_u16);
			assert_eq!(255_u8.pad_to_u16(), 255_u16);
		}
	}
}

pub mod u16 {
	use super::*;

	impl PadTo_u16 for u16 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_u16(&self) -> u16 { *self }
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_u16() {
			assert_eq!(0_u16.pad_to_u16(), 0_u16);
			assert_eq!(65535_u16.pad_to_u16(), 65535_u16);
		}
	}
}
