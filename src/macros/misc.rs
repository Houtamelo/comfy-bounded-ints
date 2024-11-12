#[doc(hidden)]
#[macro_export]
macro_rules! impl_conversions {
	(
	    $Int: ty
	    [ $N: ty ]
	    $( [$($gen:tt)*] )?
    ) => {
		#[allow(unused_imports)]
		use $crate::prelude::*;
		
		impl<T: SqueezeFrom<$N>, $( $($gen)* )?> From<$Int> for T {
			#[inline(always)]
			fn from(value: $Int) -> Self {
				Self::squeeze_from(value.get())
			}
		}
		
		impl<T: SqueezeInto<$N>, $( $($gen)* )?> From<T> for $Int {
			#[inline(always)]
			fn from(value: T) -> Self {
				Self::new(value.squeeze_into())
			}
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_neg {
    (
	    $Int: ty [ $N: ty ]
		$( [$($gen:tt)*] )?
    ) => {
	    impl<$( $($gen)* )?> std::ops::Neg for $Int {
		    type Output = $N;
		    
		    #[inline(always)]
		    fn neg(self) -> Self::Output {
			    -self.get()
		    }
	    }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_deref {
	( 
		$Int: ty [ $N: ty ]
		$( [$($gen:tt)*] )?
	) => {
		impl<$( $($gen)* )?> std::ops::Deref for $Int {
			type Target = $N;
			
			#[inline(always)]
			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}
	}
}