#[doc(hidden)]
#[macro_export]
macro_rules! impl_cmp {
    (
	    $Int: ty [ $N: ty ]
		$( [$($gen:tt)*] )?
    ) => {
	    #[allow(unused_imports)]
	    use std::cmp::*;
	    #[allow(unused_imports)]
	    use $crate::prelude::*;
	    
	    impl<T, $( $($gen)* )?> PartialEq<T> for $Int 
			where for<'a> &'a T: SqueezeInto<$N>
		{
			#[inline(always)]
			fn eq(&self, other: &T) -> bool {
				self.get() == other.squeeze_into()
			}
		}
		
		impl<T, $( $($gen)* )?> PartialEq<$Int> for T
			where for<'a> &'a T: SqueezeInto<$N>
		{
			#[inline(always)]
			fn eq(&self, other: &$Int) -> bool {
				self.squeeze_into() == other.get()
			}
		}
	    
	    impl<T, $( $($gen)* )?> PartialOrd<T> for $Int 
			where for<'a> &'a T: SqueezeInto<$N>,
	                    $Int: PartialEq<T>,
		{
			#[inline(always)]
			fn partial_cmp(&self, other: &T) -> Option<Ordering> {
				self.get().partial_cmp(&other.squeeze_into())
			}
		}
		
		impl<T, $( $($gen)* )?> PartialOrd<$Int> for T
			where for<'a> &'a T: SqueezeInto<$N>
		{
			#[inline(always)]
			fn partial_cmp(&self, other: &$Int) -> Option<Ordering> {
				self.squeeze_into().partial_cmp(&other.get())
			}
		}
    };
}