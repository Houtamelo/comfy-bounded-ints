#[doc(hidden)]
#[macro_export]
macro_rules! impl_basic_ops {
	(
		$Int: ty [ $N: ty ]
		$( [$($gen:tt)*] )?
	) => {
		#[allow(unused_imports)]
		use std::ops::*;
		#[allow(unused_imports)]
		use $crate::prelude::*;
		
		impl_basic_ops!($Int[$N] [saturating_add] Add(fn add) $( [$($gen)*] )? );
		impl_basic_ops!($Int[$N] [saturating_sub] Sub(fn sub) $( [$($gen)*] )? );
		impl_basic_ops!($Int[$N] [saturating_mul] Mul(fn mul) $( [$($gen)*] )? );
		impl_basic_ops!($Int[$N] [saturating_div] Div(fn div) $( [$($gen)*] )? );
		impl_basic_ops!($Int[$N] [rem] Rem(fn rem) $( [$($gen)*] )? );
	};
	
    (
	    $Ty: ty
	    [ $N: ty ]
	    [ $Op: ident ]
	    $Trait: ident ( fn $F: ident )
	    $( [$($gen:tt)*] )?
    ) => {
	    impl<T: SqueezeInto<$N>, $( $($gen)* )?> $Trait<T> for $Ty {
			type Output = $N;
		
			#[inline(always)]
			fn $F(self, rhs: T) -> Self::Output {
				<$N>::$Op(self.get(), rhs.squeeze_into())
			}
		}
		
		impl<T: SqueezeInto<$N>, $( $($gen)* )?> $Trait<T> for &$Ty {
			type Output = $N;
		
			#[inline(always)]
			fn $F(self, rhs: T) -> Self::Output {
				<$N>::$Op(self.get(), rhs.squeeze_into())
			}
		}
		
		impl<T: SqueezeInto<$N>, $( $($gen)* )?> $Trait<T> for &mut $Ty {
			type Output = $N;
		
			#[inline(always)]
			fn $F(self, rhs: T) -> Self::Output {
				<$N>::$Op(self.get(), rhs.squeeze_into())
			}
		}
		
		impl<T: SqueezeInto<$N>, $( $($gen)* )?> $Trait<$Ty> for T {
			type Output = $N;
		
			#[inline(always)]
			fn $F(self, rhs: $Ty) -> Self::Output {
				<$N>::$Op(self.squeeze_into(), rhs.get())
			}
		}
		
		impl<T: SqueezeInto<$N>, $( $($gen)* )?> $Trait<&$Ty> for T {
			type Output = $N;
		
			#[inline(always)]
			fn $F(self, rhs: &$Ty) -> Self::Output {
				<$N>::$Op(self.squeeze_into(), rhs.get())
			}
		}
		
		impl<T: SqueezeInto<$N>, $( $($gen)* )?> $Trait<&mut $Ty> for T {
			type Output = $N;
		
			#[inline(always)]
			fn $F(self, rhs: &mut $Ty) -> Self::Output {
				<$N>::$Op(self.squeeze_into(), rhs.get())
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
		#[allow(unused_imports)]
		use std::ops::*;
		#[allow(unused_imports)]
		use $crate::prelude::*;
		
		impl_basic_ops_assign!($Int[$N] [saturating_add][add] Add AddAssign(fn add_assign) $( [$($gen)*] )? );
		impl_basic_ops_assign!($Int[$N] [saturating_sub][sub] Sub SubAssign(fn sub_assign) $( [$($gen)*] )? );
		impl_basic_ops_assign!($Int[$N] [saturating_mul][mul] Mul MulAssign(fn mul_assign) $( [$($gen)*] )? );
		impl_basic_ops_assign!($Int[$N] [saturating_div][div] Div DivAssign(fn div_assign) $( [$($gen)*] )? );
		impl_basic_ops_assign!($Int[$N] [rem][rem] Rem RemAssign(fn rem_assign) $( [$($gen)*] )? );
	};
	
    (
	    $Ty: ty
	    [ $N: ty ]
	    [ $Op: ident ]
	    [ $Base_Op: ident ]
	    $Base_Trait: ident
	    $Trait: ident ( fn $F: ident )
	    $( [$($gen:tt)*] )?
    ) => {
	    impl<T: From<$N>, $( $($gen)* )?> $Trait<$Ty> for T
	        where for<'a> &'a mut T: SqueezeInto<$N>
	    {
			#[inline(always)]
			fn $F(&mut self, rhs: $Ty) {
				let result = <$N>::$Op(self.squeeze_into(), rhs.get());
				*self = Self::from(result)
			}
		}
		
		impl<T: From<$N>, $( $($gen)* )?> $Trait<&$Ty> for T 
	        where for<'a> &'a mut T: SqueezeInto<$N>
	    {
			#[inline(always)]
			fn $F(&mut self, rhs: &$Ty) {
				let result = <$N>::$Op(self.squeeze_into(), rhs.get());
				*self = Self::from(result)
			}
		}
		
		impl<T: From<$N>, $( $($gen)* )?> $Trait<&mut $Ty> for T 
	        where for<'a> &'a mut T: SqueezeInto<$N>
	    {
			#[inline(always)]
			fn $F(&mut self, rhs: &mut $Ty) {
				let result = <$N>::$Op(self.squeeze_into(), rhs.get());
				*self = Self::from(result)
			}
		}
		
		impl<T, $( $($gen)* )?> $Trait<T> for $Ty
			where $Ty: $Base_Trait<T, Output = $N>
		{
			#[inline(always)]
			fn $F(&mut self, rhs: T) {
				let result = <$Ty>::$Base_Op(*self, rhs);
				self.set(result);
			}
		}
    };
}