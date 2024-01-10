pub trait SqueezeTo_isize {
	fn squeeze_to_isize(&self) -> isize;
}

pub mod i8 {
	use super::*;

	impl SqueezeTo_isize for i8 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_isize(&self) -> isize {
			return *self as isize;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;
		
		#[test]
		fn test_squeeze_to_isize() {
			assert_eq!(i8::MIN.squeeze_to_isize(), i8::MIN as isize);
			assert_eq!(i8::MAX.squeeze_to_isize(), i8::MAX as isize);
			
			assert_eq!({-1_i8}.squeeze_to_isize(), -1_isize);
			assert_eq!(5_i8.squeeze_to_isize(), 5_isize);
		}
	}
}

pub mod i16 {
	use super::*;

	impl SqueezeTo_isize for i16 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_isize(&self) -> isize {
			return *self as isize;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;
		
		#[test]
		fn test_squeeze_to_isize() {
			assert_eq!(i16::MIN.squeeze_to_isize(), i16::MIN as isize);
			assert_eq!(i16::MAX.squeeze_to_isize(), i16::MAX as isize);
			
			assert_eq!({-1_i16}.squeeze_to_isize(), -1_isize);
			assert_eq!(5_i16.squeeze_to_isize(), 5_isize);
		}
	}
}

pub mod i32 {
	use super::*;

	impl SqueezeTo_isize for i32 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_isize(&self) -> isize {
			return *self as isize;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;
		
		#[test]
		fn test_squeeze_to_isize() {
			assert_eq!(i32::MIN.squeeze_to_isize(), i32::MIN as isize);
			assert_eq!(i32::MAX.squeeze_to_isize(), i32::MAX as isize);
			
			assert_eq!({-1_i32}.squeeze_to_isize(), -1_isize);
			assert_eq!(5_i32.squeeze_to_isize(), 5_isize);
		}
	}
}

#[cfg(target_pointer_width = "32")]
pub mod i64 {
	use crate::prelude::SqueezeTo_isize;

	impl SqueezeTo_isize for i64 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_isize(&self) -> isize {
			if *self < isize::MIN as i64 {
				return isize::MIN;
			} else if *self > isize::MAX as i64 {
				return isize::MAX;
			} else {
				return *self as isize;
			}
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;
		
		#[test]
		fn test_squeeze_to_isize() {
			assert_eq!(i64::MIN.squeeze_to_isize(), isize::MIN);
			assert_eq!(i64::MAX.squeeze_to_isize(), isize::MAX);
			
			assert_eq!({-1_i64}.squeeze_to_isize(), -1_isize);
			assert_eq!(5_i64.squeeze_to_isize(), 5_isize);
		}
	}
}

#[cfg(target_pointer_width = "64")]
pub mod i64 {
	use super::*;

	impl SqueezeTo_isize for i64 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_isize(&self) -> isize {
			return *self as isize;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;
		
		#[test]
		fn test_squeeze_to_isize() {
			assert_eq!(i64::MIN.squeeze_to_isize(), i64::MIN as isize);
			assert_eq!(i64::MAX.squeeze_to_isize(), i64::MAX as isize);
			
			assert_eq!({-1_i64}.squeeze_to_isize(), -1_isize);
			assert_eq!(5_i64.squeeze_to_isize(), 5_isize);
		}
	}
}

pub mod i128 {
	use super::*;

	impl SqueezeTo_isize for i128 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_isize(&self) -> isize {
			if *self < isize::MIN as i128 {
				return isize::MIN;
			} else if *self > isize::MAX as i128 {
				return isize::MAX;
			} else {
				return *self as isize;
			}
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;
		
		#[test]
		fn test_squeeze_to_isize() {
			assert_eq!(i128::MIN.squeeze_to_isize(), isize::MIN);
			assert_eq!(i128::MAX.squeeze_to_isize(), isize::MAX);
			
			assert_eq!({-1_i128}.squeeze_to_isize(), -1_isize);
			assert_eq!(5_i128.squeeze_to_isize(), 5_isize);
		}
	}
}

pub mod isize {
	use super::*;

	impl SqueezeTo_isize for isize {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_isize(&self) -> isize {
			return *self;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;
		
		#[test]
		fn test_squeeze_to_isize() {
			assert_eq!(isize::MIN.squeeze_to_isize(), isize::MIN);
			assert_eq!(isize::MAX.squeeze_to_isize(), isize::MAX);
			
			assert_eq!({-1_isize}.squeeze_to_isize(), -1_isize);
			assert_eq!(5_isize.squeeze_to_isize(), 5_isize);
		}
	}
}

pub mod u8 {
	use super::*;

	impl SqueezeTo_isize for u8 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_isize(&self) -> isize {
			return *self as isize;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;
		
		#[test]
		fn test_squeeze_to_isize() {
			assert_eq!(u8::MIN.squeeze_to_isize(), 0);
			assert_eq!(u8::MAX.squeeze_to_isize(), u8::MAX as isize);
			
			assert_eq!(5_u8.squeeze_to_isize(), 5_isize);
		}
	}
}

pub mod u16 {
	use super::*;

	impl SqueezeTo_isize for u16 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_isize(&self) -> isize {
			return *self as isize;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;
		
		#[test]
		fn test_squeeze_to_isize() {
			assert_eq!(u16::MIN.squeeze_to_isize(), 0);
			assert_eq!(u16::MAX.squeeze_to_isize(), u16::MAX as isize);
			
			assert_eq!(5_u16.squeeze_to_isize(), 5_isize);
		}
	}
}

#[cfg(target_pointer_width = "32")]
pub mod u32 {
	use crate::prelude::SqueezeTo_isize;

	impl SqueezeTo_isize for u32 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_isize(&self) -> isize {
			if *self > isize::MAX as u32 {
				return isize::MAX;
			} else {
				return *self as isize;
			}
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;
		
		#[test]
		fn test_squeeze_to_isize() {
			assert_eq!(u32::MIN.squeeze_to_isize(), 0);
			assert_eq!(u32::MAX.squeeze_to_isize(), isize::MAX);
			
			assert_eq!(5_u32.squeeze_to_isize(), 5_isize);
		}
	}
}

#[cfg(target_pointer_width = "64")]
pub mod u32 {
	use super::*;

	impl SqueezeTo_isize for u32 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_isize(&self) -> isize {
			return *self as isize;
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;
		
		#[test]
		fn test_squeeze_to_isize() {
			assert_eq!(u32::MIN.squeeze_to_isize(), 0);
			assert_eq!(u32::MAX.squeeze_to_isize(), u32::MAX as isize);
			
			assert_eq!(5_u32.squeeze_to_isize(), 5_isize);
		}
	}
}

pub mod u64 {
	use super::*;

	impl SqueezeTo_isize for u64 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_isize(&self) -> isize {
			if *self > isize::MAX as u64 {
				return isize::MAX;
			} else {
				return *self as isize;
			}
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;
		
		#[test]
		fn test_squeeze_to_isize() {
			assert_eq!(u64::MIN.squeeze_to_isize(), 0);
			assert_eq!(u64::MAX.squeeze_to_isize(), isize::MAX);
			
			assert_eq!(5_u64.squeeze_to_isize(), 5_isize);
		}
	}
}

pub mod u128 {
	use super::*;

	impl SqueezeTo_isize for u128 {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_isize(&self) -> isize {
			if *self > isize::MAX as u128 {
				return isize::MAX;
			} else {
				return *self as isize;
			}
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;
		
		#[test]
		fn test_squeeze_to_isize() {
			assert_eq!(u128::MIN.squeeze_to_isize(), 0);
			assert_eq!(u128::MAX.squeeze_to_isize(), isize::MAX);
			
			assert_eq!(5_u128.squeeze_to_isize(), 5_isize);
		}
	}
}

pub mod usize {
	use super::*;

	impl SqueezeTo_isize for usize {
		#[inline(always)]
		#[cfg_attr(feature = "no_panic", no_panic::no_panic)]
		fn squeeze_to_isize(&self) -> isize {
			if *self > isize::MAX as usize {
				return isize::MAX;
			} else {
				return *self as isize;
			}
		}
	}
	
	#[cfg(test)]
	mod tests {
		use super::*;
		
		#[test]
		fn test_squeeze_to_isize() {
			assert_eq!(usize::MIN.squeeze_to_isize(), 0);
			assert_eq!(usize::MAX.squeeze_to_isize(), isize::MAX);
			
			assert_eq!(5_usize.squeeze_to_isize(), 5_isize);
		}
	}
}
