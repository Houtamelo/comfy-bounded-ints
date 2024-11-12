use crate::prelude::*;

new_generic_bound_signed!(Int<MIN, MAX32>(i32));

#[test]
fn test() {
	#[allow(unused)]
	let mut int = Int::<0, 255>::new(5);

	macro_rules! run_tests {
		($($T: ty),* $(,)?) => {
		    $(
		        // Conversions -----------------------------------------------
		    	let primitive: $T = int.into();
		        assert_eq!(primitive, int.get() as $T);
		    
		        let other_int: Int<0, 255> = primitive.into();
		        assert_eq!(other_int.get(), int.get());
		    
		        // Arithmetic ------------------------------------------------
		        run_tests!(@BIN [+] [<$T>::squeeze_from(10)] == 15, 15);
		        run_tests!(@BIN [-] [<$T>::squeeze_from(2)] == 3);
		        run_tests!(@BIN [*] [<$T>::squeeze_from(4)] == 20, 20);
		        run_tests!(@BIN [/] [<$T>::squeeze_from(2)] == 2, 0);
		        run_tests!(@BIN [/] [<$T>::squeeze_from(10)] == 0, 2);
		        run_tests!(@BIN [%] [<$T>::squeeze_from(5)] == 0, 0);
		        run_tests!(@BIN [%] [<$T>::squeeze_from(3)] == 2, 3);
		    
		        // Assignment ------------------------------------------------
		        run_tests!(@ASG [+=] [<$T>::squeeze_from(20)] == 25);
		        run_tests!(@ASG [-=] [<$T>::squeeze_from(6)] == 0);
		        run_tests!(@ASG [*=] [<$T>::squeeze_from(1)] == 5);
		        run_tests!(@ASG [/=] [<$T>::squeeze_from(2)] == 2);
		        run_tests!(@ASG [%=] [<$T>::squeeze_from(2)] == 1);
		    
		        // Comparison ------------------------------------------------
		        run_tests!(@CMP [<$T>::squeeze_from(5)] ==, ==);
		        run_tests!(@CMP [<$T>::squeeze_from(2)] !=, !=);
		        run_tests!(@CMP [<$T>::squeeze_from(3)] >, <);
		        run_tests!(@CMP [<$T>::squeeze_from(5)] >=, <=);
		        run_tests!(@CMP [<$T>::squeeze_from(2)] >=, <);
		        run_tests!(@CMP [<$T>::squeeze_from(8)] <, >=);
		        run_tests!(@CMP [<$T>::squeeze_from(6)] <=, >);
		        run_tests!(@CMP [<$T>::squeeze_from(5)] <=, >=);
		    )*
	    };
		
		(@BIN [$Op: tt] [$Val: expr] == $Expect: expr $(, $Inverse: expr)? ) => {
			let result = int $Op $Val;
			assert_eq!(result, $Expect);
			
			let result = int $Op &$Val;
			assert_eq!(result, $Expect);
			
			let result = int $Op &mut $Val;
			assert_eq!(result, $Expect);
			
			$(
				let result = $Val $Op int;
				assert_eq!(result, $Inverse);
			
				let result = $Val $Op &int;
				assert_eq!(result, $Inverse);
			
				let result = $Val $Op &mut int;
				assert_eq!(result, $Inverse);
			)?
		};
		
		(@ASG [$Op: tt] [$Val: expr] == $Expect: expr $(, $Inverse: expr)? ) => {
			let mut result = int;
			result $Op $Val;
			assert_eq!(result.get(), $Expect);
			
			let mut result = int;
			result $Op &$Val;
			assert_eq!(result.get(), $Expect);
			
			let mut result = int;
			result $Op &mut $Val;
			assert_eq!(result.get(), $Expect);
			
			$(
				let mut result = $Val;
				result $Op int;
				assert_eq!(result, $Inverse);
			
				let mut result = $Val;
				result $Op &int;
				assert_eq!(result, $Inverse);
			
				let mut result = $Val;
				result $Op &mut int;
				assert_eq!(result, $Inverse);
			)?
		};
		
		(@CMP [$Val: expr] $Op: tt, $Inv: tt) => {
			assert!(int $Op $Val);
			assert!($Val $Inv int);
		};
	}

	run_tests!(
		i8, i16, i32, i64, i128, isize, 
		u8, u16, u32, u64, u128, usize,
	);
}