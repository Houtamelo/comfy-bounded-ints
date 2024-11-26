#[doc(hidden)]
#[macro_export]
macro_rules! impl_basic_ops_self_non_generic {
	($Int:ty[$N:ty]) => {
		impl std::ops::Add<$Int> for $Int {
			type Output = $N;

			#[inline(always)]
			fn add(self, rhs: $Int) -> Self::Output { <$N>::saturating_add(self.get(), rhs.get()) }
		}

		impl std::ops::Sub<$Int> for $Int {
			type Output = $N;

			#[inline(always)]
			fn sub(self, rhs: $Int) -> Self::Output { <$N>::saturating_sub(self.get(), rhs.get()) }
		}

		impl std::ops::Mul<$Int> for $Int {
			type Output = $N;

			#[inline(always)]
			fn mul(self, rhs: $Int) -> Self::Output { <$N>::saturating_mul(self.get(), rhs.get()) }
		}

		impl std::ops::Div<$Int> for $Int {
			type Output = $N;

			#[inline(always)]
			fn div(self, rhs: $Int) -> Self::Output { <$N>::saturating_div(self.get(), rhs.get()) }
		}

		impl std::ops::Rem<$Int> for $Int {
			type Output = $N;

			#[inline(always)]
			fn rem(self, rhs: $Int) -> Self::Output { <$N>::rem(self.get(), rhs.get()) }
		}

		impl std::iter::Step for $Int {
			fn steps_between(start: &Self, end: &Self) -> Option<usize> {
				if start < end {
					Some(
						$crate::prelude::cram::<usize>(end) - $crate::prelude::cram::<usize>(start),
					)
				} else {
					None
				}
			}

			fn forward_checked(mut start: Self, count: usize) -> Option<Self> {
				let old_start = start;
				start += count;

				let diff = start - old_start;
				if diff != 0 {
					Some(start)
				} else {
					None
				}
			}

			fn backward_checked(mut start: Self, count: usize) -> Option<Self> {
				let old_start = start;
				start -= count;

				let diff = old_start - start;
				if diff != 0 {
					Some(start)
				} else {
					None
				}
			}
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_basic_ops_self_generic {
	($Int:ident[$N:ty]) => {
		impl<const A_MIN: $N, const A_MAX: $N, const B_MIN: $N, const B_MAX: $N>
			std::ops::Add<$Int<B_MIN, B_MAX>> for $Int<A_MIN, A_MAX>
		{
			type Output = $N;

			#[inline(always)]
			fn add(self, rhs: $Int<B_MIN, B_MAX>) -> Self::Output {
				<$N>::saturating_add(self.get(), rhs.get())
			}
		}

		impl<const A_MIN: $N, const A_MAX: $N, const B_MIN: $N, const B_MAX: $N>
			std::ops::Sub<$Int<B_MIN, B_MAX>> for $Int<A_MIN, A_MAX>
		{
			type Output = $N;

			#[inline(always)]
			fn sub(self, rhs: $Int<B_MIN, B_MAX>) -> Self::Output {
				<$N>::saturating_sub(self.get(), rhs.get())
			}
		}

		impl<const A_MIN: $N, const A_MAX: $N, const B_MIN: $N, const B_MAX: $N>
			std::ops::Mul<$Int<B_MIN, B_MAX>> for $Int<A_MIN, A_MAX>
		{
			type Output = $N;

			#[inline(always)]
			fn mul(self, rhs: $Int<B_MIN, B_MAX>) -> Self::Output {
				<$N>::saturating_mul(self.get(), rhs.get())
			}
		}

		impl<const A_MIN: $N, const A_MAX: $N, const B_MIN: $N, const B_MAX: $N>
			std::ops::Div<$Int<B_MIN, B_MAX>> for $Int<A_MIN, A_MAX>
		{
			type Output = $N;

			#[inline(always)]
			fn div(self, rhs: $Int<B_MIN, B_MAX>) -> Self::Output {
				<$N>::saturating_div(self.get(), rhs.get())
			}
		}

		impl<const A_MIN: $N, const A_MAX: $N, const B_MIN: $N, const B_MAX: $N>
			std::ops::Rem<$Int<B_MIN, B_MAX>> for $Int<A_MIN, A_MAX>
		{
			type Output = $N;

			#[inline(always)]
			fn rem(self, rhs: $Int<B_MIN, B_MAX>) -> Self::Output {
				<$N>::rem(self.get(), rhs.get())
			}
		}

		impl<const MIN: $N, const MAX: $N> std::iter::Step for $Int<MIN, MAX> {
			fn steps_between(start: &Self, end: &Self) -> Option<usize> {
				if start < end {
					Some(
						$crate::prelude::cram::<usize>(end) - $crate::prelude::cram::<usize>(start),
					)
				} else {
					None
				}
			}

			fn forward_checked(mut start: Self, count: usize) -> Option<Self> {
				let old_start = start;
				start += count;

				let diff = start - old_start;
				if diff != 0 {
					Some(start)
				} else {
					None
				}
			}

			fn backward_checked(mut start: Self, count: usize) -> Option<Self> {
				let old_start = start;
				start -= count;

				let diff = old_start - start;
				if diff != 0 {
					Some(start)
				} else {
					None
				}
			}
		}
	};
}

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
	            for<'a> &'a T: $crate::prelude::CramInto<$N>,
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
	            for<'a> &'a T: $crate::prelude::CramInto<$N>
	    {
			#[inline(always)]
			fn $F(&mut self, rhs: &$Ty) {
				use $crate::prelude::CramInto;
				$( use std::ops::Rem; ${ignore($ignored)} )?

				let result = <$N>::$Op(self.cram_into(), rhs.get());
				*self = Self::from(result)
			}
		}

		impl<T, $( $($gen)* )?> std::ops::$Trait<T> for $Ty
			where T: $crate::prelude::IPrimitive + $crate::prelude::CramInto<$N>,
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
