macro_rules! signed_to_signed {
    (
	    $( [ $($Smaller: ident),* ] <<< )?
	    $I: ident
	    $( <<< [ $($Bigger: ident ),* ] )?
    ) => {
	    $($(
	        impl SqueezeInto<$Smaller> for $I {
		        #[inline(always)]
		        fn squeeze_into(self) -> $Smaller {
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
	        impl SqueezeInto<$Bigger> for $I {
		        #[inline(always)]
		        fn squeeze_into(self) -> $Bigger { self as $Bigger }
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
	        impl SqueezeInto<$Smaller> for $I {
		        #[inline(always)]
		        fn squeeze_into(self) -> $Smaller {
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
	        impl SqueezeInto<$Bigger> for $I {
		        #[inline(always)]
		        fn squeeze_into(self) -> $Bigger { 
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
	        impl SqueezeInto<$Smaller> for $I {
		        #[inline(always)]
		        fn squeeze_into(self) -> $Smaller {
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
	        impl SqueezeInto<$Bigger> for $I {
		        #[inline(always)]
		        fn squeeze_into(self) -> $Bigger { self as $Bigger }
	        }
	    )*)?
    };
}

macro_rules! impl_squeeze {
    ($($I: ident),*) => { $(impl Squeeze for $I {})* };
}

pub(crate) use {signed_to_signed, signed_to_unsigned, unsigned, impl_squeeze};