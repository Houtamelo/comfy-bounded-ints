pub trait PadTo_u64 {
	fn pad_to_u64(&self) -> u64;
}

pub mod u8 {
	use super::*;

	impl PadTo_u64 for u8 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_u64(&self) -> u64 {
			return *self as u64;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_u64() {
			assert_eq!(0_u8.pad_to_u64(), 0_u64);
			assert_eq!(255_u8.pad_to_u64(), 255_u64);
		}
	}
}

pub mod u16 {
	use super::*;

	impl PadTo_u64 for u16 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_u64(&self) -> u64 {
			return *self as u64;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_u64() {
			assert_eq!(0_u16.pad_to_u64(), 0_u64);
			assert_eq!(65535_u16.pad_to_u64(), 65535_u64);
		}
	}
}

pub mod u32 {
	use super::*;

	impl PadTo_u64 for u32 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_u64(&self) -> u64 {
			return *self as u64;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_u64() {
			assert_eq!(0_u32.pad_to_u64(), 0_u64);
			assert_eq!(4294967295_u32.pad_to_u64(), 4294967295_u64);
		}
	}
}

pub mod u64 {
	use super::*;

	impl PadTo_u64 for u64 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_u64(&self) -> u64 { *self }
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_u64() {
			assert_eq!(0_u64.pad_to_u64(), 0_u64);
			assert_eq!(18446744073709551615_u64.pad_to_u64(), 18446744073709551615_u64);
		}
	}
}

pub mod usize {
	use super::*;

	impl PadTo_u64 for usize {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_u64(&self) -> u64 {
			return *self as u64;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_u64() {
			assert_eq!(0_usize.pad_to_u64(), 0_u64);
			assert_eq!(usize::MAX.pad_to_u64(), usize::MAX as u64);
		}
	}
}
