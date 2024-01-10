pub trait PadTo_u128 {
	fn pad_to_u128(&self) -> u128;
}

pub mod u8 {
	use super::*;

	impl PadTo_u128 for u8 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_u128(&self) -> u128 {
			return *self as u128;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_u128() {
			assert_eq!(0_u8.pad_to_u128(), 0_u128);
			assert_eq!(255_u8.pad_to_u128(), 255_u128);
		}
	}
}

pub mod u16 {
	use super::*;

	impl PadTo_u128 for u16 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_u128(&self) -> u128 {
			return *self as u128;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_u128() {
			assert_eq!(0_u16.pad_to_u128(), 0_u128);
			assert_eq!(65535_u16.pad_to_u128(), 65535_u128);
		}
	}
}

pub mod u32 {
	use super::*;

	impl PadTo_u128 for u32 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_u128(&self) -> u128 {
			return *self as u128;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_u128() {
			assert_eq!(0_u32.pad_to_u128(), 0_u128);
			assert_eq!(4294967295_u32.pad_to_u128(), 4294967295_u128);
		}
	}
}

pub mod u64 {
	use super::*;

	impl PadTo_u128 for u64 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_u128(&self) -> u128 {
			return *self as u128;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_u128() {
			assert_eq!(0_u64.pad_to_u128(), 0_u128);
			assert_eq!(18446744073709551615_u64.pad_to_u128(), 18446744073709551615_u128);
		}
	}
}

pub mod u128 {
	use super::*;

	impl PadTo_u128 for u128 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_u128(&self) -> u128 { *self }
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_u128() {
			assert_eq!(0_u128.pad_to_u128(), 0_u128);
			assert_eq!(340282366920938463463374607431768211455_u128.pad_to_u128(), 340282366920938463463374607431768211455_u128);
		}
	}
}

pub mod usize {
	use super::*;

	impl PadTo_u128 for usize {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn pad_to_u128(&self) -> u128 {
			return *self as u128;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_pad_to_u128() {
			assert_eq!(0_usize.pad_to_u128(), 0_u128);
			assert_eq!(usize::MAX.pad_to_u128(), usize::MAX as u128);
		}
	}
}
