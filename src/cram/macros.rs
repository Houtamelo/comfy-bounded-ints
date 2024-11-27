macro_rules! signed_to_signed {
    (
	    $( [ $($Smaller: ident),* ] <<< )?
	    $I: ident
	    $( <<< [ $($Bigger: ident ),* ] )?
    ) => {
	    $($(
	        impl CramInto<$Smaller> for $I {
		        #[inline(always)]
		        fn cram_into(self) -> $Smaller {
			        if self < const { <$Smaller>::MIN as $I } {
				        <$Smaller>::MIN
			        } else if self > const { <$Smaller>::MAX as $I } {
				        <$Smaller>::MAX
			        } else {
				        self as $Smaller
			        }
		        }
	        }

	        impl CramInto<$Smaller> for &$I {
		        #[inline(always)]
		        fn cram_into(self) -> $Smaller {
			        if *self < const { <$Smaller>::MIN as $I } {
				        <$Smaller>::MIN
			        } else if *self > const { <$Smaller>::MAX as $I } {
				        <$Smaller>::MAX
			        } else {
				        *self as $Smaller
			        }
		        }
	        }
	    )*)?

	    $($(
	        impl CramInto<$Bigger> for $I {
		        #[inline(always)]
		        fn cram_into(self) -> $Bigger { self as $Bigger }
	        }

	        impl CramInto<$Bigger> for &$I {
		        #[inline(always)]
		        fn cram_into(self) -> $Bigger { *self as $Bigger }
	        }
	    )*)?
    };
}

macro_rules! signed_to_unsigned {
	(
	    $( [ $($Smaller: ident),* ] <<< )?
	    $I: ident
	    $( <<< [ $($Bigger: ident ),* ] )?
    ) => {
	    $($(
	        impl CramInto<$Smaller> for $I {
		        #[inline(always)]
		        fn cram_into(self) -> $Smaller {
			        if self < 0 {
				        0
			        } else if self > const { <$Smaller>::MAX as $I } {
				        <$Smaller>::MAX
			        } else {
				        self as $Smaller
			        }
		        }
	        }

	        impl CramInto<$Smaller> for &$I {
		        #[inline(always)]
		        fn cram_into(self) -> $Smaller {
			        if *self < 0 {
				        0
			        } else if *self > const { <$Smaller>::MAX as $I } {
				        <$Smaller>::MAX
			        } else {
				        *self as $Smaller
			        }
		        }
	        }
	    )*)?

	    $($(
	        impl CramInto<$Bigger> for $I {
		        #[inline(always)]
		        fn cram_into(self) -> $Bigger {
			        if self < 0 {
				        0
			        } else {
				        self as $Bigger
			        }
		        }
	        }

	        impl CramInto<$Bigger> for &$I {
		        #[inline(always)]
		        fn cram_into(self) -> $Bigger {
			        if *self < 0 {
				        0
			        } else {
				        *self as $Bigger
			        }
		        }
	        }
	    )*)?
    };
}

macro_rules! unsigned {
    (
	    $( [ $($Smaller: ident),* ] <<< )?
	    $I: ident
	    $( <<< [ $($Bigger: ident ),* ] )?
    ) => {
	    $($(
	        impl CramInto<$Smaller> for $I {
		        #[inline(always)]
		        fn cram_into(self) -> $Smaller {
			        if self <= 0 {
				        0
			        } else if self > const { <$Smaller>::MAX as $I } {
				        <$Smaller>::MAX
			        } else {
				        self as $Smaller
			        }
		        }
	        }

	        impl CramInto<$Smaller> for &$I {
		        #[inline(always)]
		        fn cram_into(self) -> $Smaller {
			        if *self <= 0 {
				        0
			        } else if *self > const { <$Smaller>::MAX as $I } {
				        <$Smaller>::MAX
			        } else {
				        *self as $Smaller
			        }
		        }
	        }
	    )*)?

	    $($(
	        impl CramInto<$Bigger> for $I {
		        #[inline(always)]
		        fn cram_into(self) -> $Bigger { self as $Bigger }
	        }

	        impl CramInto<$Bigger> for &$I {
		        #[inline(always)]
		        fn cram_into(self) -> $Bigger { *self as $Bigger }
	        }
	    )*)?
    };
}

macro_rules! cram_into_self {
    ($($I:ty),*) => {
	    $(
	        impl CramInto<$I> for $I {
		        #[inline(always)]
		        fn cram_into(self) -> $I { self }
	        }

	        impl CramInto<$I> for &$I {
		        #[inline(always)]
		        fn cram_into(self) -> $I { *self }
	        }
	    )*
    };
}

macro_rules! cram_into_floats {
	($($I:ty),*) => {
		$(impl CramInto<f32> for $I {
			#[inline(always)]
			fn cram_into(self) -> f32 { self as f32 }
		}

		impl CramInto<f32> for &$I {
			#[inline(always)]
			fn cram_into(self) -> f32 { *self as f32 }
		}

		impl CramInto<f64> for $I {
			#[inline(always)]
			fn cram_into(self) -> f64 { self as f64 }
		}

		impl CramInto<f64> for &$I {
			#[inline(always)]
			fn cram_into(self) -> f64 { *self as f64 }
		})*
	};
}

pub(crate) use cram_into_floats;
pub(crate) use cram_into_self;
pub(crate) use signed_to_signed;
pub(crate) use signed_to_unsigned;
pub(crate) use unsigned;
