pub trait SqueezeTo_usize {
	fn squeeze_to_usize(&self) -> usize;
}

pub mod i8 {
	use super::*;

	impl SqueezeTo_usize for i8 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_usize(&self) -> usize {
			if *self < 0 {
				return 0;
			} else {
				return *self as usize;
			}
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_usize() {
			assert_eq!(i8::MIN.squeeze_to_usize(), 0);
			assert_eq!(i8::MAX.squeeze_to_usize(), i8::MAX as usize);

			assert_eq!({-1_i8}.squeeze_to_usize(), 0);
			assert_eq!(5_i8.squeeze_to_usize(), 5_usize);
		}
	}
}

pub mod i16 {
	use super::*;

	impl SqueezeTo_usize for i16 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_usize(&self) -> usize {
			if *self < 0 {
				return 0;
			} else {
				return *self as usize;
			}
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_usize() {
			assert_eq!(i16::MIN.squeeze_to_usize(), 0);
			assert_eq!(i16::MAX.squeeze_to_usize(), i16::MAX as usize);

			assert_eq!({-1_i16}.squeeze_to_usize(), 0);
			assert_eq!(5_i16.squeeze_to_usize(), 5_usize);
		}
	}
}

pub mod i32 {
	use super::*;

	impl SqueezeTo_usize for i32 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_usize(&self) -> usize {
			if *self < 0 {
				return 0;
			} else {
				return *self as usize;
			}
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_usize() {
			assert_eq!(i32::MIN.squeeze_to_usize(), 0);
			assert_eq!(i32::MAX.squeeze_to_usize(), i32::MAX as usize);

			assert_eq!({-1_i32}.squeeze_to_usize(), 0);
			assert_eq!(5_i32.squeeze_to_usize(), 5_usize);
		}
	}
}

#[cfg(target_pointer_width = "32")]
pub mod i64 {
	use crate::prelude::SqueezeTo_usize;

	impl SqueezeTo_usize for i64 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_usize(&self) -> usize {
			if *self < 0 {
				return 0;
			} else if *self > usize::MAX as i64 {
				return usize::MAX;
			} else {
				return *self as usize;
			}
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_usize() {
			assert_eq!(i64::MIN.squeeze_to_usize(), 0);
			assert_eq!(i64::MAX.squeeze_to_usize(), usize::MAX);

			assert_eq!({-1_i64}.squeeze_to_usize(), 0);
			assert_eq!(5_i64.squeeze_to_usize(), 5_usize);
		}
	}
}

#[cfg(target_pointer_width = "64")]
pub mod i64 {
	use super::*;

	impl SqueezeTo_usize for i64 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_usize(&self) -> usize {
			if *self < 0 {
				return 0;
			} else {
				return *self as usize;
			}
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_usize() {
			assert_eq!(i64::MIN.squeeze_to_usize(), 0);
			assert_eq!(i64::MAX.squeeze_to_usize(), i64::MAX as usize);

			assert_eq!({-1_i64}.squeeze_to_usize(), 0);
			assert_eq!(5_i64.squeeze_to_usize(), 5_usize);
		}
	}
}

pub mod i128 {
	use super::*;

	impl SqueezeTo_usize for i128 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_usize(&self) -> usize {
			if *self < 0 {
				return 0;
			} else if *self > usize::MAX as i128 {
				return usize::MAX;
			} else {
				return *self as usize;
			}
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_usize() {
			assert_eq!(i128::MIN.squeeze_to_usize(), 0);
			assert_eq!(i128::MAX.squeeze_to_usize(), usize::MAX);

			assert_eq!({-1_i128}.squeeze_to_usize(), 0);
			assert_eq!(5_i128.squeeze_to_usize(), 5_usize);
		}
	}
}

pub mod isize {
	use super::*;

	impl SqueezeTo_usize for isize {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_usize(&self) -> usize {
			if *self < 0 {
				return 0;
			} else {
				return *self as usize;
			}
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_usize() {
			assert_eq!(isize::MIN.squeeze_to_usize(), 0);
			assert_eq!(isize::MAX.squeeze_to_usize(), isize::MAX as usize);
			
			assert_eq!({-1_isize}.squeeze_to_usize(), 0);
			assert_eq!(5_isize.squeeze_to_usize(), 5_usize);
		}
	}
}

pub mod u8 {
	use super::*;

	impl SqueezeTo_usize for u8 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_usize(&self) -> usize {
			return *self as usize;
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_usize() {
			assert_eq!(u8::MIN.squeeze_to_usize(), 0);
			assert_eq!(u8::MAX.squeeze_to_usize(), u8::MAX as usize);

			assert_eq!(5_u8.squeeze_to_usize(), 5_usize);
		}
	}
}

pub mod u16 {
	use super::*;

	impl SqueezeTo_usize for u16 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_usize(&self) -> usize {
			return *self as usize;
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_usize() {
			assert_eq!(u16::MIN.squeeze_to_usize(), 0);
			assert_eq!(u16::MAX.squeeze_to_usize(), u16::MAX as usize);

			assert_eq!(5_u16.squeeze_to_usize(), 5_usize);
		}
	}
}

pub mod u32 {
	use super::*;

	impl SqueezeTo_usize for u32 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_usize(&self) -> usize {
			return *self as usize;
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_usize() {
			assert_eq!(u32::MIN.squeeze_to_usize(), 0);
			assert_eq!(u32::MAX.squeeze_to_usize(), u32::MAX as usize);

			assert_eq!(5_u32.squeeze_to_usize(), 5_usize);
		}
	}
}

#[cfg(target_pointer_width = "32")]
pub mod u64 {
	use crate::prelude::SqueezeTo_usize;

	impl SqueezeTo_usize for u64 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_usize(&self) -> usize {
			if *self > usize::MAX as u64 {
				return usize::MAX;
			} else {
				return *self as usize;
			}
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_usize() {
			assert_eq!(u64::MIN.squeeze_to_usize(), 0);
			assert_eq!(u64::MAX.squeeze_to_usize(), usize::MAX);

			assert_eq!(5_u64.squeeze_to_usize(), 5_usize);
		}
	}
}

#[cfg(target_pointer_width = "64")]
pub mod u64 {
	use super::*;

	impl SqueezeTo_usize for u64 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_usize(&self) -> usize {
			return *self as usize;
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_usize() {
			assert_eq!(u64::MIN.squeeze_to_usize(), 0);
			assert_eq!(u64::MAX.squeeze_to_usize(), u64::MAX as usize);

			assert_eq!(5_u64.squeeze_to_usize(), 5_usize);
		}
	}
}

pub mod u128 {
	use super::*;

	impl SqueezeTo_usize for u128 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_usize(&self) -> usize {
			if *self > usize::MAX as u128 {
				return usize::MAX;
			} else {
				return *self as usize;
			}
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_usize() {
			assert_eq!(u128::MIN.squeeze_to_usize(), 0);
			assert_eq!(u128::MAX.squeeze_to_usize(), usize::MAX);

			assert_eq!(5_u128.squeeze_to_usize(), 5_usize);
		}
	}
}

pub mod usize {
	use super::*;

	impl SqueezeTo_usize for usize {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_usize(&self) -> usize {
			return *self;
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_squeeze_to_usize() {
			assert_eq!(usize::MIN.squeeze_to_usize(), usize::MIN);
			assert_eq!(usize::MAX.squeeze_to_usize(), usize::MAX);

			assert_eq!(5_usize.squeeze_to_usize(), 5_usize);
		}
	}
}
