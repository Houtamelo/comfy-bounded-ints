pub trait SqueezeTo_u128 {
	fn squeeze_to_u128(&self) -> u128;
}

pub mod i8 {
	use super::*;

	impl SqueezeTo_u128 for i8 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_u128(&self) -> u128 {
			if *self < 0 {
				return 0;
			} else {
				return *self as u128;
			}
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_u128() {
			assert_eq!(i8::MIN.squeeze_to_u128(), 0);
			assert_eq!(i8::MAX.squeeze_to_u128(), i8::MAX as u128);

			assert_eq!({-1_i8}.squeeze_to_u128(), 0);
			assert_eq!(5_i8.squeeze_to_u128(), 5_u128);
		}
	}
}

pub mod i16 {
	use super::*;

	impl SqueezeTo_u128 for i16 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_u128(&self) -> u128 {
			if *self < 0 {
				return 0;
			} else {
				return *self as u128;
			}
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_u128() {
			assert_eq!(i16::MIN.squeeze_to_u128(), 0);
			assert_eq!(i16::MAX.squeeze_to_u128(), i16::MAX as u128);

			assert_eq!({-1_i16}.squeeze_to_u128(), 0);
			assert_eq!(5_i16.squeeze_to_u128(), 5_u128);
		}
	}
}

pub mod i32 {
	use super::*;

	impl SqueezeTo_u128 for i32 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_u128(&self) -> u128 {
			if *self < 0 {
				return 0;
			} else {
				return *self as u128;
			}
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_u128() {
			assert_eq!(i32::MIN.squeeze_to_u128(), 0);
			assert_eq!(i32::MAX.squeeze_to_u128(), i32::MAX as u128);

			assert_eq!({-1_i32}.squeeze_to_u128(), 0);
			assert_eq!(5_i32.squeeze_to_u128(), 5_u128);
		}
	}
}

pub mod i64 {
	use super::*;

	impl SqueezeTo_u128 for i64 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_u128(&self) -> u128 {
			if *self < 0 {
				return 0;
			} else {
				return *self as u128;
			}
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_u128() {
			assert_eq!(i64::MIN.squeeze_to_u128(), 0);
			assert_eq!(i64::MAX.squeeze_to_u128(), i64::MAX as u128);

			assert_eq!({-1_i64}.squeeze_to_u128(), 0);
			assert_eq!(5_i64.squeeze_to_u128(), 5_u128);
		}
	}
}

pub mod i128 {
	use super::*;

	impl SqueezeTo_u128 for i128 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_u128(&self) -> u128 {
			if *self < 0 {
				return 0;
			} else {
				return *self as u128;
			}
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_u128() {
			assert_eq!(i128::MIN.squeeze_to_u128(), 0);
			assert_eq!(i128::MAX.squeeze_to_u128(), i128::MAX as u128);

			assert_eq!({-1_i128}.squeeze_to_u128(), 0);
			assert_eq!(5_i128.squeeze_to_u128(), 5_u128);
		}
	}
}

pub mod isize {
	use super::*;

	impl SqueezeTo_u128 for isize {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_u128(&self) -> u128 {
			if *self < 0 {
				return 0;
			} else {
				return *self as u128;
			}
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_u128() {
			assert_eq!(isize::MIN.squeeze_to_u128(), 0);
			assert_eq!(isize::MAX.squeeze_to_u128(), isize::MAX as u128);

			assert_eq!({-1_isize}.squeeze_to_u128(), 0);
			assert_eq!(5_isize.squeeze_to_u128(), 5_u128);
		}
	}
}

pub mod u8 {
	use super::*;

	impl SqueezeTo_u128 for u8 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_u128(&self) -> u128 {
			return *self as u128;
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_u128() {
			assert_eq!(u8::MIN.squeeze_to_u128(), 0);
			assert_eq!(u8::MAX.squeeze_to_u128(), u8::MAX as u128);

			assert_eq!(5_u8.squeeze_to_u128(), 5_u128);
		}
	}
}

pub mod u16 {
	use super::*;

	impl SqueezeTo_u128 for u16 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_u128(&self) -> u128 {
			return *self as u128;
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_u128() {
			assert_eq!(u16::MIN.squeeze_to_u128(), 0);
			assert_eq!(u16::MAX.squeeze_to_u128(), u16::MAX as u128);

			assert_eq!(5_u16.squeeze_to_u128(), 5_u128);
		}
	}
}

pub mod u32 {
	use super::*;

	impl SqueezeTo_u128 for u32 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_u128(&self) -> u128 {
			return *self as u128;
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_u128() {
			assert_eq!(u32::MIN.squeeze_to_u128(), 0);
			assert_eq!(u32::MAX.squeeze_to_u128(), u32::MAX as u128);

			assert_eq!(5_u32.squeeze_to_u128(), 5_u128);
		}
	}
}

pub mod u64 {
	use super::*;

	impl SqueezeTo_u128 for u64 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_u128(&self) -> u128 {
			return *self as u128;
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_u128() {
			assert_eq!(u64::MIN.squeeze_to_u128(), 0);
			assert_eq!(u64::MAX.squeeze_to_u128(), u64::MAX as u128);

			assert_eq!(5_u64.squeeze_to_u128(), 5_u128);
		}
	}
}

pub mod u128 {
	use super::*;

	impl SqueezeTo_u128 for u128 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_u128(&self) -> u128 {
			return *self;
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_u128() {
			assert_eq!(u128::MIN.squeeze_to_u128(), 0);
			assert_eq!(u128::MAX.squeeze_to_u128(), u128::MAX);

			assert_eq!(5_u128.squeeze_to_u128(), 5_u128);
		}
	}
}

pub mod usize {
	use super::*;

	impl SqueezeTo_u128 for usize {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_u128(&self) -> u128 {
			return *self as u128;
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_u128() {
			assert_eq!(usize::MIN.squeeze_to_u128(), 0);
			assert_eq!(usize::MAX.squeeze_to_u128(), usize::MAX as u128);

			assert_eq!(5_usize.squeeze_to_u128(), 5_u128);
		}
	}
}
