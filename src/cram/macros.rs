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
	    )*)?

	    $($(
	        impl CramInto<$Bigger> for $I {
		        #[inline(always)]
		        fn cram_into(self) -> $Bigger { self as $Bigger }
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
	    )*)?

	    $($(
	        impl CramInto<$Bigger> for $I {
		        #[inline(always)]
		        fn cram_into(self) -> $Bigger { self as $Bigger }
	        }
	    )*)?
    };
}

macro_rules! impl_cram {
    ($($I: ident),*) => { $(impl Cram for $I {})* };
}

pub(crate) use impl_cram;
pub(crate) use signed_to_signed;
pub(crate) use signed_to_unsigned;
pub(crate) use unsigned;
