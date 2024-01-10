pub trait PadTo_u32 {
	fn pad_to_u32(&self) -> u32;
}

pub mod u8 {
	use super::*;

	impl PadTo_u32 for u8 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_u32(&self) -> u32 {
			return *self as u32;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_u32() {
			assert_eq!(0_u8.pad_to_u32(), 0_u32);
			assert_eq!(255_u8.pad_to_u32(), 255_u32);
		}
	}
}

pub mod u16 {
	use super::*;

	impl PadTo_u32 for u16 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_u32(&self) -> u32 {
			return *self as u32;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_u32() {
			assert_eq!(0_u16.pad_to_u32(), 0_u32);
			assert_eq!(65535_u16.pad_to_u32(), 65535_u32);
		}
	}
}

pub mod u32 {
	use super::*;

	impl PadTo_u32 for u32 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_u32(&self) -> u32 { *self }
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_u32() {
			assert_eq!(0_u32.pad_to_u32(), 0_u32);
			assert_eq!(4294967295_u32.pad_to_u32(), 4294967295_u32);
		}
	}
}

#[cfg(target_pointer_width = "32")]
pub mod usize {
	use super::*;

	impl PadTo_u32 for usize {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_u32(&self) -> u32 {
			return *self as u32;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_u32() {
			assert_eq!(0_usize.pad_to_u32(), 0_u32);
			assert_eq!(usize::MAX.pad_to_u32(), usize::MAX as u32);
		}
	}
}
