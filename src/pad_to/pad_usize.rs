pub trait PadTo_usize {
	fn pad_to_usize(&self) -> usize;
}

pub mod u8 {
	use super::*;

	impl PadTo_usize for u8 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_usize(&self) -> usize {
			return *self as usize;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_usize() {
			assert_eq!(0_u8.pad_to_usize(), 0_usize);
			assert_eq!(255_u8.pad_to_usize(), 255_usize);
		}
	}
}

pub mod u16 {
	use super::*;

	impl PadTo_usize for u16 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_usize(&self) -> usize {
			return *self as usize;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_usize() {
			assert_eq!(0_u16.pad_to_usize(), 0_usize);
			assert_eq!(65535_u16.pad_to_usize(), 65535_usize);
		}
	}
}

pub mod u32 {
	use super::*;

	impl PadTo_usize for u32 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_usize(&self) -> usize {
			return *self as usize;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_usize() {
			assert_eq!(0_u32.pad_to_usize(), 0_usize);
			assert_eq!(4294967295_u32.pad_to_usize(), 4294967295_usize);
		}
	}
}

#[cfg(target_pointer_width = "64")]
pub mod u64 {
	use super::*;

	impl PadTo_usize for u64 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_usize(&self) -> usize {
			return *self as usize;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_usize() {
			assert_eq!(0_u64.pad_to_usize(), 0_usize);
			assert_eq!(18446744073709551615_u64.pad_to_usize(), 18446744073709551615_usize);
		}
	}
}

pub mod usize {
	use super::*;

	impl PadTo_usize for usize {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_usize(&self) -> usize { *self }
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_usize() {
			assert_eq!(0_usize.pad_to_usize(), 0_usize);
			assert_eq!(usize::MAX.pad_to_usize(), usize::MAX);
		}
	}
}
