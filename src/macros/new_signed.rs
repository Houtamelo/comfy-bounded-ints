#[macro_export]
macro_rules! new_generic_bound_signed {
    ($Int: ident <$MIN: ident, $MAX: ident> ($N: ty)) => {
	    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
	    #[cfg_attr(feature = "serde", serde(transparent))]
	    #[derive(Debug, Copy, Clone, Eq, Hash, Ord)]
	    #[repr(transparent)]
		pub struct $Int<const $MIN: $N, const $MAX: $N>($N);

		impl<const $MIN: $N, const $MAX: $N> $Int<$MIN, $MAX> {
		    #[inline(always)]
		    pub const fn new(inner: $N) -> Self {
				let val =
					if inner < $MIN {
						$MIN
					} else if inner > $MAX {
						$MAX
					} else {
						inner
					};

				Self(val)
			}

		    #[inline(always)]
			pub const fn get(&self) -> $N { self.0 }

		    #[inline(always)]
			pub const fn set(&mut self, inner: $N) { *self = Self::new(inner) }
		}

	    impl<const $MIN: $N, const $MAX: $N> Default for $Int<$MIN, $MAX> {
		    #[inline(always)]
		    fn default() -> Self {
			    Self::new(0)
		    }
	    }

	    $crate::prelude::impl_basic_ops!($Int<$MIN, $MAX>[$N] [ const $MIN: $N, const $MAX: $N ]);
	    $crate::prelude::impl_basic_ops_assign!($Int<$MIN, $MAX>[$N] [ const $MIN: $N, const $MAX: $N ]);
	    $crate::prelude::impl_neg!($Int<$MIN, $MAX>[$N] [ const $MIN: $N, const $MAX: $N ]);
	    $crate::prelude::impl_conversions!($Int, $Int<$MIN, $MAX> [$N] [ const $MIN: $N, const $MAX: $N ]);
	    $crate::prelude::impl_deref!($Int<$MIN, $MAX>[$N] [ const $MIN: $N, const $MAX: $N ]);
	    $crate::prelude::impl_cmp!($Int, $Int<$MIN, $MAX>[$N] [ const $MIN: $N, const $MAX: $N ]);
	    $crate::prelude::impl_display!($Int<$MIN, $MAX>[$N] [ const $MIN: $N, const $MAX: $N ]);
    };
}

#[macro_export]
macro_rules! new_bound_signed {
	($Int:ident($N:ty)[$L:expr, $R:expr]) => {
		#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
		#[cfg_attr(feature = "serde", serde(transparent))]
		#[derive(Debug, Copy, Clone, Eq, Hash, Ord)]
		#[repr(transparent)]
		pub struct $Int($N);

		impl $Int {
			pub const MIN: $N = $L;
			pub const MAX: $N = $R;

			#[inline(always)]
			pub const fn new(inner: $N) -> Self {
				let val = if inner < Self::MIN {
					Self::MIN
				} else if inner > Self::MAX {
					Self::MAX
				} else {
					inner
				};

				Self(val)
			}

			#[inline(always)]
			pub const fn get(&self) -> $N { self.0 }

			#[inline(always)]
			pub const fn set(&mut self, inner: $N) { *self = Self::new(inner) }
		}

		impl Default for $Int {
			#[inline(always)]
			fn default() -> Self { Self::new(0) }
		}

		$crate::prelude::impl_basic_ops!($Int[$N]);
		$crate::prelude::impl_basic_ops_assign!($Int[$N]);
		$crate::prelude::impl_neg!($Int[$N]);
		$crate::prelude::impl_conversions!($Int, $Int[$N]);
		$crate::prelude::impl_deref!($Int[$N]);
		$crate::prelude::impl_cmp!($Int, $Int[$N]);
		$crate::prelude::impl_display!($Int[$N]);
	};
}
