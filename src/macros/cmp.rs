#[doc(hidden)]
#[macro_export]
macro_rules! impl_cmp {
    (
	    $Int: ty [ $N: ty ]
		$( [$($gen:tt)*] )?
    ) => {
	    impl<T, $( $($gen)* )?> std::cmp::PartialEq<T> for $Int
				where $N: $crate::prelude::EncapsulatesBoth<T, Out: std::cmp::PartialEq + for<'a> $crate::prelude::CramFrom<&'a T> + $crate::prelude::CramFrom<$N>>,
		{
			#[inline(always)]
			fn eq(&self, other: &T) -> bool {
				use $crate::prelude::{EncapsulatesBoth, CramFrom};
				let out_a = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<$N>>::cram_from(self.get());
				let out_b = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<&T>>::cram_from(other);
				out_a.eq(&out_b)
			}
		}

	    impl<T, $( $($gen)* )?> std::cmp::PartialEq<$Int> for T
				where $N: $crate::prelude::EncapsulatesBoth<T, Out: std::cmp::PartialEq + for<'a> $crate::prelude::CramFrom<&'a T> + $crate::prelude::CramFrom<$N>>,
		{
			#[inline(always)]
			fn eq(&self, other: &$Int) -> bool {
				use $crate::prelude::{EncapsulatesBoth, CramFrom};
				let out_a = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<$N>>::cram_from(other.get());
				let out_b = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<&T>>::cram_from(self);
				out_a.eq(&out_b)
			}
		}

	    impl<T, $( $($gen)* )?> std::cmp::PartialOrd<T> for $Int
			where $N: $crate::prelude::EncapsulatesBoth<T, Out: std::cmp::PartialOrd + for<'a> $crate::prelude::CramFrom<&'a T> + $crate::prelude::CramFrom<$N>>,
		{
			#[inline(always)]
			fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
				use $crate::prelude::{EncapsulatesBoth, CramFrom};
				let out_a = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<$N>>::cram_from(self.get());
				let out_b = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<&T>>::cram_from(other);
				out_a.partial_cmp(&out_b)
			}
		}

		impl<T, $( $($gen)* )?> std::cmp::PartialOrd<$Int> for T
			where $N: $crate::prelude::EncapsulatesBoth<T, Out: std::cmp::PartialOrd + for<'a> $crate::prelude::CramFrom<&'a T> + $crate::prelude::CramFrom<$N>>,
		{
			#[inline(always)]
			fn partial_cmp(&self, other: &$Int) -> Option<std::cmp::Ordering> {
				use $crate::prelude::{EncapsulatesBoth, CramFrom};
				let out_a = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<$N>>::cram_from(other.get());
				let out_b = <<$N as EncapsulatesBoth<T>>::Out as CramFrom<&T>>::cram_from(self);
				out_b.partial_cmp(&out_a)
			}
		}
    };
}
