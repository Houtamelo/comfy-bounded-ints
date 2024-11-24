use crate::prelude::*;

new_generic_bound_signed!(Int<MIN, MAX32>(i32));

#[test]
fn test() {
	#[allow(unused)]
	let mut int = Int::<0, 255>::new(5);
	
	macro_rules! test_primitives {
		($($T: ty),* $(,)?) => {
		    $(
		        // Conversions -----------------------------------------------
		    	let primitive: $T = int.into();
		        assert_eq!(primitive, int.get() as $T);
		    
		        let other_int: Int<0, 255> = primitive.into();
		        assert_eq!(other_int.get(), int.get());
		    
		        // Assignment ------------------------------------------------
		        test_primitives!(@ASG $T [+=] [20] == 25);
		        test_primitives!(@ASG $T [-=] [6] == 0);
		        test_primitives!(@ASG $T [*=] [1] == 5);
		        test_primitives!(@ASG $T [/=] [2] == 2);
		        test_primitives!(@ASG $T [%=] [2] == 1);
		    
		        // Comparison ------------------------------------------------
		        test_primitives!(@CMP $T [5] ==, ==);
		        test_primitives!(@CMP $T [2] !=, !=);
		        test_primitives!(@CMP $T [3] >, <);
		        test_primitives!(@CMP $T [5] >=, <=);
		        test_primitives!(@CMP $T [2] >=, <);
		        test_primitives!(@CMP $T [8] <, >=);
		        test_primitives!(@CMP $T [6] <=, >);
		        test_primitives!(@CMP $T [5] <=, >=);
		    )*
	    };
		
		(@BIN $T: ty [$Op: tt] [$Val: expr] == $Expect: expr $(, $Inverse: expr)? ) => {
			let val = <$T>::cram_from($Val);
			
			let result = int $Op val.clone();
			assert_eq!(result, $Expect);
			
			let result = int $Op &val.clone();
			assert_eq!(result, $Expect);
			
			let result = int $Op &mut val.clone();
			assert_eq!(result, $Expect);
			
			$(
				let result = val.clone() $Op int;
				assert_eq!(result, $Inverse);
			
				let result = val.clone() $Op &int;
				assert_eq!(result, $Inverse);
			
				let result = val.clone() $Op &mut int;
				assert_eq!(result, $Inverse);
			)?
		};
		
		(@ASG $T: ty [$Op: tt] [$Val: expr] == $Expect: expr $(, $Inverse: expr)? ) => {
			let val = <$T>::cram_from($Val);
			
			let mut result = int;
			result $Op val.clone();
			assert_eq!(result.cram::<$T>(), $Expect);
			
			let mut result = int;
			result $Op &val.clone();
			assert_eq!(result.cram::<$T>(), $Expect);
			
			let mut result = int;
			result $Op &mut val.clone();
			assert_eq!(result.cram::<$T>(), $Expect);
			
			$(
				let mut result = val.clone();
				result $Op int;
				assert_eq!(result, $Inverse);
			
				let mut result = val.clone();
				result $Op &int;
				assert_eq!(result, $Inverse);
			
				let mut result = val.clone();
				result $Op &mut int;
				assert_eq!(result, $Inverse);
			)?
		};
		
		(@CMP $T: ty [$Val: expr] $Op: tt, $Inv: tt) => {
			let val = <$T>::cram_from($Val);
			
			assert!(int $Op val.clone());
			
			if !(val.clone() $Inv int) {
				panic!("Assertion failed, values: {} {} {}", val.clone(), stringify!($Inv), int);
			}
		};
	}
	

	test_primitives!(
		i8, i16, i32, i64, i128, isize, 
		u8, u16, u32, u64, u128, usize,
	);
}