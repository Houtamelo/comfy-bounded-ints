#[doc(hidden)]
#[macro_export]
macro_rules! impl_conversions {
	(
		$Ident: ident,
	    $Int: ty
	    [ $N: ty ]
	    $( [$($gen:tt)*] )?
    ) => {
		impl<T: ${concat(Not, $Ident)} + $crate::IPrimitive + $crate::prelude::EncapsulatesBoth<$N>, $( $($gen)* )?> $crate::prelude::EncapsulatesBoth<$Int> for T {
			type Out = T::Out;
		}

		impl<T, $( $($gen)* )?> $crate::prelude::EncapsulatesBoth<T> for $Int
			where $N: $crate::prelude::EncapsulatesBoth<T> {
			type Out = <$N as $crate::prelude::EncapsulatesBoth<T>>::Out;
		}

		pub auto trait ${concat(Not, $Ident)} {}
		impl<$( $($gen)* )?> !${concat(Not, $Ident)} for $Int {}

		impl<T: $crate::prelude::IPrimitive + $crate::prelude::CramFrom<$N>, $( $($gen)* )?> From<$Int> for T {
			#[inline(always)]
			fn from(value: $Int) -> Self {
				Self::cram_from(value.get())
			}
		}

		impl<T: $crate::prelude::IPrimitive + $crate::prelude::CramFrom<$N>, $( $($gen)* )?> From<&$Int> for T {
			#[inline(always)]
			fn from(value: &$Int) -> Self {
				Self::cram_from(value.get())
			}
		}

		impl<T: $crate::prelude::IPrimitive + $crate::prelude::CramFrom<$N>, $( $($gen)* )?> From<&mut $Int> for T {
			#[inline(always)]
			fn from(value: &mut $Int) -> Self {
				Self::cram_from(value.get())
			}
		}

		impl<T: ${concat(Not, $Ident)} + $crate::prelude::CramInto<$N>, $( $($gen)* )?> From<T> for $Int {
			#[inline(always)]
			fn from(value: T) -> Self {
				Self::new(value.cram_into())
			}
		}

		impl<$( $($gen)* )?> From<&$Int> for $Int {
			#[inline(always)]
			fn from(value: &$Int) -> Self {
				Self::new(value.get())
			}
		}

		impl<$( $($gen)* )?> From<&mut $Int> for $Int {
			#[inline(always)]
			fn from(value: &mut $Int) -> Self {
				Self::new(value.get())
			}
		}

		impl<T: $crate::prelude::IPrimitive + $crate::prelude::CramFrom<$N>, $( $($gen)* )?> $crate::prelude::CramInto<T> for $Int {
			#[inline(always)]
			fn cram_into(self) -> T {
				T::cram_from(self.get())
			}
		}

		impl<T: $crate::prelude::IPrimitive + $crate::prelude::CramFrom<$N>, $( $($gen)* )?> $crate::prelude::CramInto<T> for &$Int {
			#[inline(always)]
			fn cram_into(self) -> T {
				T::cram_from(self.get())
			}
		}

		impl<T: $crate::prelude::IPrimitive + $crate::prelude::CramFrom<$N>, $( $($gen)* )?> $crate::prelude::CramInto<T> for &mut $Int {
			#[inline(always)]
			fn cram_into(self) -> T {
				T::cram_from(self.get())
			}
		}

		impl<T: $crate::prelude::CramInto<$N>, $( $($gen)* )?> $crate::prelude::CramInto<$Int> for T {
			#[inline(always)]
			fn cram_into(self) -> $Int {
				<$Int>::new(self.cram_into())
			}
		}

		impl<$( $($gen)* )?> !$crate::prelude::IPrimitive for $Int {}
		//impl<$( $($gen)* )?> !$crate::prelude::IPrimitive for &$Int {}
		//impl<$( $($gen)* )?> !$crate::prelude::IPrimitive for &mut $Int {}
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
