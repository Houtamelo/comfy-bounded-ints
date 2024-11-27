#[doc(hidden)]
#[macro_export]
macro_rules! impl_cmp {
    (
	    $Ident: ident,
	    $Int: ty [ $N: ty ]
		$( [$($gen:tt)*] )?
    ) => {
	    impl<T: $crate::INotRef + $crate::IPrimitive, $( $($gen)* )?> std::cmp::PartialEq<T> for $Int
				where $N: $crate::EncapsulatesBoth<T, Out: std::cmp::PartialEq + for<'a> $crate::prelude::CramFrom<&'a T> + $crate::prelude::CramFrom<$N>>,
		{
			#[inline(always)]
			fn eq(&self, other: &T) -> bool {
				use $crate::prelude::CramFrom;
				use $crate::EncapsulatesBoth;
				let out_a = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<$N>>::cram_from(self.get());
				let out_b = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<&T>>::cram_from(other);
				out_a.eq(&out_b)
			}
		}

	    impl<T: $crate::INotRef + $crate::IPrimitive, $( $($gen)* )?> std::cmp::PartialEq<&T> for $Int
				where $N: $crate::EncapsulatesBoth<T, Out: std::cmp::PartialEq + for<'a> $crate::prelude::CramFrom<&'a T> + $crate::prelude::CramFrom<$N>>,
		{
			#[inline(always)]
			fn eq(&self, other: &&T) -> bool {
				use $crate::prelude::CramFrom;
				use $crate::EncapsulatesBoth;
				let out_a = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<$N>>::cram_from(self.get());
				let out_b = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<&T>>::cram_from(other);
				out_a.eq(&out_b)
			}
		}

	    impl<T: $crate::INotRef + $crate::IPrimitive, $( $($gen)* )?> std::cmp::PartialEq<&mut T> for $Int
				where $N: $crate::EncapsulatesBoth<T, Out: std::cmp::PartialEq + for<'a> $crate::prelude::CramFrom<&'a T> + $crate::prelude::CramFrom<$N>>,
		{
			#[inline(always)]
			fn eq(&self, other: &&mut T) -> bool {
				use $crate::prelude::CramFrom;
				use $crate::EncapsulatesBoth;
				let out_a = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<$N>>::cram_from(self.get());
				let out_b = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<&T>>::cram_from(other);
				out_a.eq(&out_b)
			}
		}

	    impl<T: $crate::INotRef, $( $($gen)* )?> std::cmp::PartialEq<T> for &$Int
				where $N: $crate::EncapsulatesBoth<T, Out: std::cmp::PartialEq + for<'a> $crate::prelude::CramFrom<&'a T> + $crate::prelude::CramFrom<$N>>,
		{
			#[inline(always)]
			fn eq(&self, other: &T) -> bool {
				use $crate::prelude::CramFrom;
				use $crate::EncapsulatesBoth;
				let out_a = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<$N>>::cram_from(self.get());
				let out_b = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<&T>>::cram_from(other);
				out_a.eq(&out_b)
			}
		}

	    impl<T: $crate::INotRef, $( $($gen)* )?> std::cmp::PartialEq<T> for &mut $Int
				where $N: $crate::EncapsulatesBoth<T, Out: std::cmp::PartialEq + for<'a> $crate::prelude::CramFrom<&'a T> + $crate::prelude::CramFrom<$N>>,
		{
			#[inline(always)]
			fn eq(&self, other: &T) -> bool {
				use $crate::prelude::CramFrom;
				use $crate::EncapsulatesBoth;
				let out_a = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<$N>>::cram_from(self.get());
				let out_b = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<&T>>::cram_from(other);
				out_a.eq(&out_b)
			}
		}

	    impl<T: $crate::INotRef, $( $($gen)* )?> std::cmp::PartialEq<$Int> for T
				where $N: $crate::EncapsulatesBoth<T, Out: std::cmp::PartialEq + for<'a> $crate::prelude::CramFrom<&'a T> + $crate::prelude::CramFrom<$N>>,
		{
			#[inline(always)]
			fn eq(&self, other: &$Int) -> bool {
				use $crate::prelude::CramFrom;
				use $crate::EncapsulatesBoth;
				let out_a = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<$N>>::cram_from(other.get());
				let out_b = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<&T>>::cram_from(self);
				out_a.eq(&out_b)
			}
		}

	    impl<T: $crate::INotRef, $( $($gen)* )?> std::cmp::PartialEq<&$Int> for T
				where $N: $crate::EncapsulatesBoth<T, Out: std::cmp::PartialEq + for<'a> $crate::prelude::CramFrom<&'a T> + $crate::prelude::CramFrom<$N>>,
		{
			#[inline(always)]
			fn eq(&self, other: &&$Int) -> bool {
				use $crate::prelude::CramFrom;
				use $crate::EncapsulatesBoth;
				let out_a = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<$N>>::cram_from(other.get());
				let out_b = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<&T>>::cram_from(self);
				out_a.eq(&out_b)
			}
		}

	    impl<T: $crate::INotRef, $( $($gen)* )?> std::cmp::PartialEq<&mut $Int> for T
				where $N: $crate::EncapsulatesBoth<T, Out: std::cmp::PartialEq + for<'a> $crate::prelude::CramFrom<&'a T> + $crate::prelude::CramFrom<$N>>,
		{
			#[inline(always)]
			fn eq(&self, other: &&mut $Int) -> bool {
				use $crate::prelude::CramFrom;
				use $crate::EncapsulatesBoth;
				let out_a = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<$N>>::cram_from(other.get());
				let out_b = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<&T>>::cram_from(self);
				out_a.eq(&out_b)
			}
		}

	    impl<T: $crate::INotRef + $crate::IPrimitive, $( $($gen)* )?> std::cmp::PartialEq<$Int> for &T
				where $N: $crate::EncapsulatesBoth<T, Out: std::cmp::PartialEq + for<'a> $crate::prelude::CramFrom<&'a T> + $crate::prelude::CramFrom<$N>>,
		{
			#[inline(always)]
			fn eq(&self, other: &$Int) -> bool {
				use $crate::prelude::CramFrom;
				use $crate::EncapsulatesBoth;
				let out_a = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<$N>>::cram_from(other.get());
				let out_b = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<&T>>::cram_from(self);
				out_a.eq(&out_b)
			}
		}

	    impl<T: $crate::INotRef + $crate::IPrimitive, $( $($gen)* )?> std::cmp::PartialEq<$Int> for &mut T
				where $N: $crate::EncapsulatesBoth<T, Out: std::cmp::PartialEq + for<'a> $crate::prelude::CramFrom<&'a T> + $crate::prelude::CramFrom<$N>>,
		{
			#[inline(always)]
			fn eq(&self, other: &$Int) -> bool {
				use $crate::prelude::CramFrom;
				use $crate::EncapsulatesBoth;
				let out_a = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<$N>>::cram_from(other.get());
				let out_b = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<&T>>::cram_from(self);
				out_a.eq(&out_b)
			}
		}

	    impl<T: $crate::INotRef + $crate::IPrimitive, $( $($gen)* )?> std::cmp::PartialOrd<T> for $Int
			where $N: $crate::EncapsulatesBoth<T, Out: std::cmp::PartialOrd + for<'a> $crate::prelude::CramFrom<&'a T> + $crate::prelude::CramFrom<$N>>,
		{
			#[inline(always)]
			fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
				use $crate::prelude::CramFrom;
				use $crate::EncapsulatesBoth;
				let out_a = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<$N>>::cram_from(self.get());
				let out_b = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<&T>>::cram_from(other);
				out_a.partial_cmp(&out_b)
			}
		}

	    impl<T: $crate::INotRef + $crate::IPrimitive, $( $($gen)* )?> std::cmp::PartialOrd<&T> for $Int
			where $N: $crate::EncapsulatesBoth<T, Out: std::cmp::PartialOrd + for<'a> $crate::prelude::CramFrom<&'a T> + $crate::prelude::CramFrom<$N>>,
		{
			#[inline(always)]
			fn partial_cmp(&self, other: &&T) -> Option<std::cmp::Ordering> {
				use $crate::prelude::CramFrom;
				use $crate::EncapsulatesBoth;
				let out_a = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<$N>>::cram_from(self.get());
				let out_b = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<&T>>::cram_from(other);
				out_a.partial_cmp(&out_b)
			}
		}

	    impl<T: $crate::INotRef + $crate::IPrimitive, $( $($gen)* )?> std::cmp::PartialOrd<&mut T> for $Int
			where $N: $crate::EncapsulatesBoth<T, Out: std::cmp::PartialOrd + for<'a> $crate::prelude::CramFrom<&'a T> + $crate::prelude::CramFrom<$N>>,
		{
			#[inline(always)]
			fn partial_cmp(&self, other: &&mut T) -> Option<std::cmp::Ordering> {
				use $crate::prelude::CramFrom;
				use $crate::EncapsulatesBoth;
				let out_a = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<$N>>::cram_from(self.get());
				let out_b = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<&T>>::cram_from(other);
				out_a.partial_cmp(&out_b)
			}
		}

	    impl<T: $crate::INotRef, $( $($gen)* )?> std::cmp::PartialOrd<T> for &$Int
			where $N: $crate::EncapsulatesBoth<T, Out: std::cmp::PartialOrd + for<'a> $crate::prelude::CramFrom<&'a T> + $crate::prelude::CramFrom<$N>>,
		{
			#[inline(always)]
			fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
				use $crate::prelude::CramFrom;
				use $crate::EncapsulatesBoth;
				let out_a = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<$N>>::cram_from(self.get());
				let out_b = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<&T>>::cram_from(other);
				out_a.partial_cmp(&out_b)
			}
		}

	    impl<T: $crate::INotRef, $( $($gen)* )?> std::cmp::PartialOrd<T> for &mut $Int
			where $N: $crate::EncapsulatesBoth<T, Out: std::cmp::PartialOrd + for<'a> $crate::prelude::CramFrom<&'a T> + $crate::prelude::CramFrom<$N>>,
		{
			#[inline(always)]
			fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
				use $crate::prelude::CramFrom;
				use $crate::EncapsulatesBoth;
				let out_a = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<$N>>::cram_from(self.get());
				let out_b = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<&T>>::cram_from(other);
				out_a.partial_cmp(&out_b)
			}
		}

	    impl<T: $crate::INotRef, $( $($gen)* )?> std::cmp::PartialOrd<&mut T> for &$Int
			where $N: $crate::EncapsulatesBoth<T, Out: std::cmp::PartialOrd + for<'a> $crate::prelude::CramFrom<&'a T> + $crate::prelude::CramFrom<$N>>,
	            for<'a, 'b> &'a $Int: std::cmp::PartialEq<&'b mut T>,
		{
			#[inline(always)]
			fn partial_cmp(&self, other: &&mut T) -> Option<std::cmp::Ordering> {
				use $crate::prelude::CramFrom;
				use $crate::EncapsulatesBoth;
				let out_a = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<$N>>::cram_from(self.get());
				let out_b = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<&T>>::cram_from(other);
				out_a.partial_cmp(&out_b)
			}
		}

	    impl<T: $crate::INotRef, $( $($gen)* )?> std::cmp::PartialOrd<&T> for &mut $Int
			where $N: $crate::EncapsulatesBoth<T, Out: std::cmp::PartialOrd + for<'a> $crate::prelude::CramFrom<&'a T> + $crate::prelude::CramFrom<$N>>,
	            for<'a, 'b> &'a mut $Int: std::cmp::PartialEq<&'b T>,
		{
			#[inline(always)]
			fn partial_cmp(&self, other: &&T) -> Option<std::cmp::Ordering> {
				use $crate::prelude::CramFrom;
				use $crate::EncapsulatesBoth;
				let out_a = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<$N>>::cram_from(self.get());
				let out_b = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<&T>>::cram_from(other);
				out_a.partial_cmp(&out_b)
			}
		}

		impl<T: $crate::INotRef, $( $($gen)* )?> std::cmp::PartialOrd<$Int> for T
			where $N: $crate::EncapsulatesBoth<T, Out: std::cmp::PartialOrd + for<'a> $crate::prelude::CramFrom<&'a T> + $crate::prelude::CramFrom<$N>>,
		{
			#[inline(always)]
			fn partial_cmp(&self, other: &$Int) -> Option<std::cmp::Ordering> {
				use $crate::prelude::CramFrom;
				use $crate::EncapsulatesBoth;
				let out_a = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<$N>>::cram_from(other.get());
				let out_b = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<&T>>::cram_from(self);
				out_b.partial_cmp(&out_a)
			}
		}

	    impl<T: $crate::INotRef, $( $($gen)* )?> std::cmp::PartialOrd<&$Int> for T
			where $N: $crate::EncapsulatesBoth<T, Out: std::cmp::PartialOrd + for<'a> $crate::prelude::CramFrom<&'a T> + $crate::prelude::CramFrom<$N>>,
		{
			#[inline(always)]
			fn partial_cmp(&self, other: &&$Int) -> Option<std::cmp::Ordering> {
				use $crate::prelude::CramFrom;
				use $crate::EncapsulatesBoth;
				let out_a = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<$N>>::cram_from(other.get());
				let out_b = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<&T>>::cram_from(self);
				out_b.partial_cmp(&out_a)
			}
		}

	    impl<T: $crate::INotRef, $( $($gen)* )?> std::cmp::PartialOrd<&mut $Int> for T
			where $N: $crate::EncapsulatesBoth<T, Out: std::cmp::PartialOrd + for<'a> $crate::prelude::CramFrom<&'a T> + $crate::prelude::CramFrom<$N>>,
		{
			#[inline(always)]
			fn partial_cmp(&self, other: &&mut $Int) -> Option<std::cmp::Ordering> {
				use $crate::prelude::CramFrom;
				use $crate::EncapsulatesBoth;
				let out_a = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<$N>>::cram_from(other.get());
				let out_b = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<&T>>::cram_from(self);
				out_b.partial_cmp(&out_a)
			}
		}

	    impl<T: $crate::INotRef + $crate::IPrimitive, $( $($gen)* )?> std::cmp::PartialOrd<$Int> for &T
			where $N: $crate::EncapsulatesBoth<T, Out: std::cmp::PartialOrd + for<'a> $crate::prelude::CramFrom<&'a T> + $crate::prelude::CramFrom<$N>>,
		{
			#[inline(always)]
			fn partial_cmp(&self, other: &$Int) -> Option<std::cmp::Ordering> {
				use $crate::prelude::CramFrom;
				use $crate::EncapsulatesBoth;
				let out_a = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<$N>>::cram_from(other.get());
				let out_b = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<&T>>::cram_from(self);
				out_b.partial_cmp(&out_a)
			}
		}

	    impl<T: $crate::INotRef + $crate::IPrimitive, $( $($gen)* )?> std::cmp::PartialOrd<&mut $Int> for &T
			where $N: $crate::EncapsulatesBoth<T, Out: std::cmp::PartialOrd + for<'a> $crate::prelude::CramFrom<&'a T> + $crate::prelude::CramFrom<$N>>,
		{
			#[inline(always)]
			fn partial_cmp(&self, other: &&mut $Int) -> Option<std::cmp::Ordering> {
				use $crate::prelude::CramFrom;
				use $crate::EncapsulatesBoth;
				let out_a = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<$N>>::cram_from(other.get());
				let out_b = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<&T>>::cram_from(self);
				out_b.partial_cmp(&out_a)
			}
		}

	    impl<T: $crate::INotRef + $crate::IPrimitive, $( $($gen)* )?> std::cmp::PartialOrd<$Int> for &mut T
			where $N: $crate::EncapsulatesBoth<T, Out: std::cmp::PartialOrd + for<'a> $crate::prelude::CramFrom<&'a T> + $crate::prelude::CramFrom<$N>>,
		{
			#[inline(always)]
			fn partial_cmp(&self, other: &$Int) -> Option<std::cmp::Ordering> {
				use $crate::prelude::CramFrom;
				use $crate::EncapsulatesBoth;
				let out_a = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<$N>>::cram_from(other.get());
				let out_b = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<&T>>::cram_from(self);
				out_b.partial_cmp(&out_a)
			}
		}

	    impl<T: $crate::INotRef + $crate::IPrimitive, $( $($gen)* )?> std::cmp::PartialOrd<&$Int> for &mut T
			where $N: $crate::EncapsulatesBoth<T, Out: std::cmp::PartialOrd + for<'a> $crate::prelude::CramFrom<&'a T> + $crate::prelude::CramFrom<$N>>,
		{
			#[inline(always)]
			fn partial_cmp(&self, other: &&$Int) -> Option<std::cmp::Ordering> {
				use $crate::prelude::CramFrom;
				use $crate::EncapsulatesBoth;
				let out_a = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<$N>>::cram_from(other.get());
				let out_b = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<&T>>::cram_from(self);
				out_b.partial_cmp(&out_a)
			}
		}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_float_cmp_eq {
    (
	    $Float: ty,
	    $Ident: ident,
	    $Int: ty [ $N: ty ]
		$( [$($gen:tt)*] )?
    ) => {
	    impl<$( $($gen)* )?> std::cmp::PartialEq<$Float> for $Int {
		    #[inline(always)]
		    fn eq(&self, other: &$Float) -> bool {
			    <$Float>::eq(&(self.get() as $Float), other)
		    }
	    }

	    impl<$( $($gen)* )?> std::cmp::PartialEq<&$Float> for $Int {
		    #[inline(always)]
		    fn eq(&self, other: &&$Float) -> bool {
			    <$Float>::eq(&(self.get() as $Float), *other)
		    }
	    }

	    impl<$( $($gen)* )?> std::cmp::PartialEq<&mut $Float> for $Int {
		    #[inline(always)]
		    fn eq(&self, other: &&mut $Float) -> bool {
			    <$Float>::eq(&(self.get() as $Float), *other)
		    }
	    }

	    impl<$( $($gen)* )?> std::cmp::PartialEq<$Float> for &$Int {
		    #[inline(always)]
		    fn eq(&self, other: &$Float) -> bool {
			    <$Float>::eq(&(self.get() as $Float), other)
		    }
	    }

	    impl<$( $($gen)* )?> std::cmp::PartialEq<$Float> for &mut $Int {
		    #[inline(always)]
		    fn eq(&self, other: &$Float) -> bool {
			    <$Float>::eq(&(self.get() as $Float), other)
		    }
	    }

	    impl<$( $($gen)* )?> std::cmp::PartialEq<$Int> for $Float {
		    #[inline(always)]
		    fn eq(&self, other: &$Int) -> bool {
			    <$Float>::eq(&(other.get() as $Float), self)
		    }
	    }

	    impl<$( $($gen)* )?> std::cmp::PartialEq<&$Int> for $Float {
		    #[inline(always)]
		    fn eq(&self, other: &&$Int) -> bool {
			    <$Float>::eq(&(other.get() as $Float), self)
		    }
	    }

	    impl<$( $($gen)* )?> std::cmp::PartialEq<&mut $Int> for $Float {
		    #[inline(always)]
		    fn eq(&self, other: &&mut $Int) -> bool {
			    <$Float>::eq(&(other.get() as $Float), self)
		    }
	    }

	    impl<$( $($gen)* )?> std::cmp::PartialEq<$Int> for &$Float {
		    #[inline(always)]
		    fn eq(&self, other: &$Int) -> bool {
			    <$Float>::eq(&(other.get() as $Float), *self)
		    }
	    }

	    impl<$( $($gen)* )?> std::cmp::PartialEq<$Int> for &mut $Float {
		    #[inline(always)]
		    fn eq(&self, other: &$Int) -> bool {
			    <$Float>::eq(&(other.get() as $Float), *self)
		    }
	    }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_float_cmp_ord {
    (
	    $Float: ty,
	    $Ident: ident,
	    $Int: ty [ $N: ty ]
		$( [$($gen:tt)*] )?
    ) => {
	    impl<$( $($gen)* )?> std::cmp::PartialOrd<$Float> for $Int {
		    #[inline(always)]
		    fn partial_cmp(&self, other: &$Float) -> Option<std::cmp::Ordering> {
			    <$Float>::partial_cmp(&(self.get() as $Float), other)
		    }
	    }

	    impl<$( $($gen)* )?> std::cmp::PartialOrd<&$Float> for $Int {
		    #[inline(always)]
		    fn partial_cmp(&self, other: &&$Float) -> Option<std::cmp::Ordering> {
			    <$Float>::partial_cmp(&(self.get() as $Float), *other)
		    }
	    }

	    impl<$( $($gen)* )?> std::cmp::PartialOrd<&mut $Float> for $Int {
		    #[inline(always)]
		    fn partial_cmp(&self, other: &&mut $Float) -> Option<std::cmp::Ordering> {
			    <$Float>::partial_cmp(&(self.get() as $Float), *other)
		    }
	    }

	    impl<$( $($gen)* )?> std::cmp::PartialOrd<$Float> for &$Int {
		    #[inline(always)]
		    fn partial_cmp(&self, other: &$Float) -> Option<std::cmp::Ordering> {
			    <$Float>::partial_cmp(&(self.get() as $Float), other)
		    }
	    }

	    impl<$( $($gen)* )?> std::cmp::PartialOrd<&mut $Float> for &$Int {
		    #[inline(always)]
		    fn partial_cmp(&self, other: &&mut $Float) -> Option<std::cmp::Ordering> {
			    <$Float>::partial_cmp(&(self.get() as $Float), *other)
		    }
	    }

	    impl<$( $($gen)* )?> std::cmp::PartialOrd<$Float> for &mut $Int {
		    #[inline(always)]
		    fn partial_cmp(&self, other: &$Float) -> Option<std::cmp::Ordering> {
			    <$Float>::partial_cmp(&(self.get() as $Float), other)
		    }
	    }

	    impl<$( $($gen)* )?> std::cmp::PartialOrd<&$Float> for &mut $Int {
		    #[inline(always)]
		    fn partial_cmp(&self, other: &&$Float) -> Option<std::cmp::Ordering> {
			    <$Float>::partial_cmp(&(self.get() as $Float), *other)
		    }
	    }

	    impl<$( $($gen)* )?> std::cmp::PartialOrd<$Int> for $Float {
		    #[inline(always)]
		    fn partial_cmp(&self, other: &$Int) -> Option<std::cmp::Ordering> {
			    <$Float>::partial_cmp(&(other.get() as $Float), self)
		    }
	    }

	    impl<$( $($gen)* )?> std::cmp::PartialOrd<&$Int> for $Float {
		    #[inline(always)]
		    fn partial_cmp(&self, other: &&$Int) -> Option<std::cmp::Ordering> {
			    <$Float>::partial_cmp(&(other.get() as $Float), self)
		    }
	    }

	    impl<$( $($gen)* )?> std::cmp::PartialOrd<&mut $Int> for $Float {
		    #[inline(always)]
		    fn partial_cmp(&self, other: &&mut $Int) -> Option<std::cmp::Ordering> {
			    <$Float>::partial_cmp(&(other.get() as $Float), self)
		    }
	    }

	    impl<$( $($gen)* )?> std::cmp::PartialOrd<$Int> for &$Float {
		    #[inline(always)]
		    fn partial_cmp(&self, other: &$Int) -> Option<std::cmp::Ordering> {
			    <$Float>::partial_cmp(&(other.get() as $Float), *self)
		    }
	    }

	    impl<$( $($gen)* )?> std::cmp::PartialOrd<&mut $Int> for &$Float {
		    #[inline(always)]
		    fn partial_cmp(&self, other: &&mut $Int) -> Option<std::cmp::Ordering> {
			    <$Float>::partial_cmp(&(other.get() as $Float), *self)
		    }
	    }

	    impl<$( $($gen)* )?> std::cmp::PartialOrd<$Int> for &mut $Float {
		    #[inline(always)]
		    fn partial_cmp(&self, other: &$Int) -> Option<std::cmp::Ordering> {
			    <$Float>::partial_cmp(&(other.get() as $Float), *self)
		    }
	    }

	    impl<$( $($gen)* )?> std::cmp::PartialOrd<&$Int> for &mut $Float {
		    #[inline(always)]
		    fn partial_cmp(&self, other: &&$Int) -> Option<std::cmp::Ordering> {
			    <$Float>::partial_cmp(&(other.get() as $Float), *self)
		    }
	    }
    };
}
