#[doc(hidden)]
#[macro_export]
macro_rules! impl_conversions {
	(
	    $Int: ty
	    [ $N: ty ]
	    $( [$($gen:tt)*] )?
    ) => {
		impl<T: $crate::prelude::CramFrom<$N>, $( $($gen)* )?> From<$Int> for T {
			#[inline(always)]
			fn from(value: $Int) -> Self {
				Self::cram_from(value.get())
			}
		}
		
		impl<T: $crate::prelude::CramInto<$N>, $( $($gen)* )?> From<T> for $Int {
			#[inline(always)]
			fn from(value: T) -> Self {
				Self::new(value.cram_into())
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

#[doc(hidden)]
#[macro_export]
macro_rules! impl_display {
    (
	    $Int: ty [ $N: ty ]
		$( [$($gen:tt)*] )?
    ) => {
	    impl<$( $($gen)* )?> std::fmt::Display for $Int {
		    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			    <$N as std::fmt::Display>::fmt(&self.get(), f)
		    }
	    }
    };
}