#[doc(hidden)]
#[macro_export]
macro_rules! impl_basic_ops {
	(
		$Int: ty [ $N: ty ]
		$( [$($gen:tt)*] )?
	) => {
		$crate::prelude::impl_basic_ops!($Int[$N] [saturating_add] Add(fn add) $( [$($gen)*] )? );
		$crate::prelude::impl_basic_ops!($Int[$N] [saturating_sub] Sub(fn sub) $( [$($gen)*] )? );
		$crate::prelude::impl_basic_ops!($Int[$N] [saturating_mul] Mul(fn mul) $( [$($gen)*] )? );
		$crate::prelude::impl_basic_ops!($Int[$N] [saturating_div] Div(fn div) $( [$($gen)*] )? );
		$crate::prelude::impl_basic_ops!($Int[$N] [rem] Rem(fn rem) $( [$($gen)*] )? );
	};
	
    (
	    $Ty: ty
	    [ $N: ty ]
	    [ $Op: ident ]
	    $Trait: ident ( fn $F: ident )
	    $( [$($gen:tt)*] )?
    ) => {
	    impl<$( $($gen)* )?> std::ops::$Trait<$N> for $Ty {
			type Output = $N;
		
			#[inline(always)]
			fn $F(self, rhs: $N) -> Self::Output {
				<$N>::$Op(self.get(), rhs)
			}
		}
		
		impl<$( $($gen)* )?> std::ops::$Trait<$N> for &$Ty {
			type Output = $N;
		
			#[inline(always)]
			fn $F(self, rhs: $N) -> Self::Output {
				<$N>::$Op(self.get(), rhs)
			}
		}
		
		impl<$( $($gen)* )?> std::ops::$Trait<$N> for &mut $Ty {
			type Output = $N;
		
			#[inline(always)]
			fn $F(self, rhs: $N) -> Self::Output {
				<$N>::$Op(self.get(), rhs)
			}
		}
		
		impl<$( $($gen)* )?> std::ops::$Trait<$Ty> for $N {
			type Output = $N;
		
			#[inline(always)]
			fn $F(self, rhs: $Ty) -> Self::Output {
				<$N>::$Op(self, rhs.get())
			}
		}
		
		impl<$( $($gen)* )?> std::ops::$Trait<&$Ty> for $N {
			type Output = $N;
		
			#[inline(always)]
			fn $F(self, rhs: &$Ty) -> Self::Output {
				<$N>::$Op(self, rhs.get())
			}
		}
	    
	    impl<$( $($gen)* )?> std::ops::$Trait<&mut $Ty> for $N {
			type Output = $N;
		
			#[inline(always)]
			fn $F(self, rhs: &mut $Ty) -> Self::Output {
				<$N>::$Op(self, rhs.get())
			}
		}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_basic_ops_assign {
	(
		$Int: ty [ $N: ty ]
		$( [$($gen:tt)*] )?
	) => {
		$crate::prelude::impl_basic_ops_assign!($Int[$N] [saturating_add] AddAssign(fn add_assign) $( [$($gen)*] )? );
		$crate::prelude::impl_basic_ops_assign!($Int[$N] [saturating_sub] SubAssign(fn sub_assign) $( [$($gen)*] )? );
		$crate::prelude::impl_basic_ops_assign!($Int[$N] [saturating_mul] MulAssign(fn mul_assign) $( [$($gen)*] )? );
		$crate::prelude::impl_basic_ops_assign!($Int[$N] [saturating_div] DivAssign(fn div_assign) $( [$($gen)*] )? );
		$crate::prelude::impl_basic_ops_assign!($Int[$N] [rem] @REM; RemAssign(fn rem_assign) $( [$($gen)*] )? );
	};
	
    (
	    $Ty: ty
	    [ $N: ty ]
	    [ $Op: ident ]
	    $( @REM $ignored: tt )?
	    $Trait: ident ( fn $F: ident )
	    $( [$($gen:tt)*] )?
    ) => {
	    impl<T, $( $($gen)* )?> std::ops::$Trait<$Ty> for T
	        where T: From<$N>,
	            for<'a> &'a mut T: $crate::prelude::CramInto<$N>,
	    {
			#[inline(always)]
			fn $F(&mut self, rhs: $Ty) {
				use $crate::prelude::CramInto;
				$( use std::ops::Rem; ${ignore($ignored)} )?
		
				let result = <$N>::$Op(self.cram_into(), rhs.get());
				*self = Self::from(result)
			}
		}
		
		impl<T, $( $($gen)* )?> std::ops::$Trait<&$Ty> for T 
	        where T: From<$N>, 
	            for<'a> &'a mut T: $crate::prelude::CramInto<$N>
	    {
			#[inline(always)]
			fn $F(&mut self, rhs: &$Ty) {
				use $crate::prelude::CramInto;
				$( use std::ops::Rem; ${ignore($ignored)} )?
			
				let result = <$N>::$Op(self.cram_into(), rhs.get());
				*self = Self::from(result)
			}
		}
		
		impl<T, $( $($gen)* )?> std::ops::$Trait<&mut $Ty> for T 
	        where T: From<$N>, 
	            for<'a> &'a mut T: $crate::prelude::CramInto<$N>,
	    {
			#[inline(always)]
			fn $F(&mut self, rhs: &mut $Ty) {
				use $crate::prelude::CramInto;
				$( use std::ops::Rem; ${ignore($ignored)} )?
				
				let result = <$N>::$Op(self.cram_into(), rhs.get());
				*self = Self::from(result)
			}
		}
		
		impl<T, $( $($gen)* )?> std::ops::$Trait<T> for $Ty
			where T: $crate::prelude::CramInto<$N>,
		{
			#[inline(always)]
			fn $F(&mut self, rhs: T) {
				$( use std::ops::Rem; ${ignore($ignored)} )?
	
				let result = <$N>::$Op(self.get(), rhs.cram_into());
				self.set(result);
			}
		}
    };
}