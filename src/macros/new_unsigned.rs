#[macro_export]
macro_rules! new_bound_unsigned {
    ($Int: ident ($N: ty)[ $L: expr, $R: expr ]) => {
	    #[allow(unused_imports)]
		use std::ops::*;
		#[allow(unused_imports)]
		use $crate::prelude::*;
	    
	    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
	    #[repr(transparent)]
		pub struct $Int($N);
		
		impl $Int {
		    pub const MIN: $N = $L;
		    pub const MAX: $N = $R;
		    
		    #[inline(always)]
			pub const fn new(inner: $N) -> Self {
				let val =
					if inner < Self::MIN {
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
		    fn default() -> Self {
			    Self::new(0)
		    }
	    }
	    
	    impl_basic_ops!($Int[$N]);
	    impl_basic_ops_assign!($Int[$N]);
	    impl_conversions!($Int[$N]);
	    impl_deref!($Int[$N]);
	    impl_cmp!($Int[$N]);
    };
}

#[macro_export]
macro_rules! new_generic_bound_unsigned {
    ($Int: ident <$MIN: ident, $MAX: ident> ($N: ty)) => {
	    #[allow(unused_imports)]
		use std::ops::*;
		#[allow(unused_imports)]
		use $crate::prelude::*;
	    
	    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
	    
	    impl_basic_ops!($Int<$MIN, $MAX>[$N] [ const $MIN: $N, const $MAX: $N ]);
	    impl_basic_ops_assign!($Int<$MIN, $MAX>[$N] [ const $MIN: $N, const $MAX: $N ]);
	    impl_conversions!($Int<$MIN, $MAX> [$N] [ const $MIN: $N, const $MAX: $N ]);
	    impl_deref!($Int<$MIN, $MAX>[$N] [ const $MIN: $N, const $MAX: $N ]);
	    impl_cmp!($Int<$MIN, $MAX>[$N] [ const $MIN: $N, const $MAX: $N ]);
    };
}