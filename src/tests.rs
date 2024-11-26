use crate::prelude::*;

#[test]
fn test() {
	#[allow(unused)]
	let mut int = Int::new(5);

	macro_rules! test_primitives {
		($($T: ty),* $(,)?) => {
		    $(
		        // Conversions -----------------------------------------------
		    	let primitive: $T = int.into();
		        assert_eq!(primitive, int.get() as $T);

		        let other_int: Int = primitive.into();
		        assert_eq!(other_int.get(), int.get());

		        // Assignment ------------------------------------------------
		        test_primitives!(@ASG $T [+=] [20] == 25);
		        test_primitives!(@ASG $T [-=] [5] == 0);
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
			assert_eq!(cram::<$T>(result), $Expect);

			let mut result = int;
			result $Op &val.clone();
			assert_eq!(cram::<$T>(result), $Expect);

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
		i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize,
	);
}

//noinspection RsTraitObligations
#[test]
fn compiles() {
	macro_rules! from {
	    ($($Val: expr),* $(,)?) => {
		    $(
		        let _ = Int::from($Val);
			    let _ = Int::from(&$Val);
		        let _ = Int::cram_from($Val);
		        let _ = Int::cram_from(&$Val);
		    )*

	    };
	}

	from!(
		0,
		0_i8,
		0_i16,
		0_i32,
		0_i64,
		0_i128,
		0_isize,
		0_u8,
		0_u16,
		0_u32,
		0_u64,
		0_u128,
		0_usize,
		Int::new(5),
		UInt::new(5),
	);

	macro_rules! into {
			($($N: ty),* $(,)?) => {
				$(
					let _: $N = Int::new(5).into();
					let _: $N = (&Int::new(5)).into();
					let _: $N = Int::new(5).cram_into();
					let _: $N = (&Int::new(5)).cram_into();
				)*
			};
		}

	into!(
		i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, Int, UInt
	);

	let mut int = Int::new(5);

	macro_rules! cmp {
	    ($($Val: expr),* $(,)?) => {
		    $(
		        let _ = int < $Val;
			    let _ = int > $Val;
			    let _ = int <= $Val;
			    let _ = int >= $Val;
		        let _ = int == $Val;
		        let _ = int != $Val;

		        let _ = $Val < int;
			    let _ = $Val > int;
			    let _ = $Val <= int;
			    let _ = $Val >= int;
		        let _ = $Val == int;
		        let _ = $Val != int;

			    //---------------

		        let _ = &int < $Val;
			    let _ = &int > $Val;
			    let _ = &int <= $Val;
			    let _ = &int >= $Val;
		        let _ = &int == $Val;
		        let _ = &int != $Val;

		        let _ = $Val < &int;
			    let _ = $Val > &int;
			    let _ = $Val <= &int;
			    let _ = $Val >= &int;
		        let _ = $Val == &int;
		        let _ = $Val != &int;

			    //---------------

		        let _ = &mut int < $Val;
			    let _ = &mut int > $Val;
			    let _ = &mut int <= $Val;
			    let _ = &mut int >= $Val;
		        let _ = &mut int == $Val;
		        let _ = &mut int != $Val;

		        let _ = $Val < &mut int;
			    let _ = $Val > &mut int;
			    let _ = $Val <= &mut int;
			    let _ = $Val >= &mut int;
		        let _ = $Val == &mut int;
		        let _ = $Val != &mut int;

			    //---------------

			    let _ = int < &$Val;
			    let _ = int > &$Val;
			    let _ = int <= &$Val;
			    let _ = int >= &$Val;
		        let _ = int == &$Val;
		        let _ = int != &$Val;

		        let _ = &$Val < int;
			    let _ = &$Val > int;
			    let _ = &$Val <= int;
			    let _ = &$Val >= int;
		        let _ = &$Val == int;
		        let _ = &$Val != int;

			    //---------------

		        let _ = &int < &$Val;
			    let _ = &int > &$Val;
			    let _ = &int <= &$Val;
			    let _ = &int >= &$Val;
		        let _ = &int == &$Val;
		        let _ = &int != &$Val;

		        let _ = &$Val < &int;
			    let _ = &$Val > &int;
			    let _ = &$Val <= &int;
			    let _ = &$Val >= &int;
		        let _ = &$Val == &int;
		        let _ = &$Val != &int;

			    //---------------

		        let _ = &mut int < &$Val;
			    let _ = &mut int > &$Val;
			    let _ = &mut int <= &$Val;
			    let _ = &mut int >= &$Val;
		        let _ = &mut int == &$Val;
		        let _ = &mut int != &$Val;

		        let _ = &$Val < &mut int;
			    let _ = &$Val > &mut int;
			    let _ = &$Val <= &mut int;
			    let _ = &$Val >= &mut int;
		        let _ = &$Val == &mut int;
		        let _ = &$Val != &mut int;

			    //---------------

		        let _ = int < &mut $Val;
			    let _ = int > &mut $Val;
			    let _ = int <= &mut $Val;
			    let _ = int >= &mut $Val;
		        let _ = int == &mut $Val;
		        let _ = int != &mut $Val;

		        let _ = &mut $Val < int;
			    let _ = &mut $Val > int;
			    let _ = &mut $Val <= int;
			    let _ = &mut $Val >= int;
		        let _ = &mut $Val == int;
		        let _ = &mut $Val != int;

			    //---------------

		        let _ = &int < &mut $Val;
			    let _ = &int > &mut $Val;
			    let _ = &int <= &mut $Val;
			    let _ = &int >= &mut $Val;
		        let _ = &int == &mut $Val;
		        let _ = &int != &mut $Val;

		        let _ = &mut $Val < &int;
			    let _ = &mut $Val > &int;
			    let _ = &mut $Val <= &int;
			    let _ = &mut $Val >= &int;
		        let _ = &mut $Val == &int;
		        let _ = &mut $Val != &int;

			    //---------------

		        let _ = &mut int < &mut $Val;
			    let _ = &mut int > &mut $Val;
			    let _ = &mut int <= &mut $Val;
			    let _ = &mut int >= &mut $Val;
		        let _ = &mut int == &mut $Val;
		        let _ = &mut int != &mut $Val;

		        let _ = &mut $Val < &mut int;
			    let _ = &mut $Val > &mut int;
			    let _ = &mut $Val <= &mut int;
			    let _ = &mut $Val >= &mut int;
		        let _ = &mut $Val == &mut int;
		        let _ = &mut $Val != &mut int;

		        //---------------
		    )*
	    };
	}

	cmp!(
		3,
		3_i8,
		3_i16,
		3_i32,
		3_i64,
		3_i128,
		3_isize,
		3_u8,
		3_u16,
		3_u32,
		3_u64,
		3_u128,
		3_usize,
		Int::new(5),
		UInt::new(5),
	);

	macro_rules! assign {
		($($Val: expr),* $(,)?) => {
			$(
				int += $Val;
				int -= $Val;
				int *= $Val;
				int /= $Val;
				int %= $Val;

				int += &$Val;
				int -= &$Val;
				int *= &$Val;
				int /= &$Val;
				int %= &$Val;
			)*
		};
	}

	assign!(
		3,
		3_i8,
		3_i16,
		3_i32,
		3_i64,
		3_i128,
		3_isize,
		3_u8,
		3_u16,
		3_u32,
		3_u64,
		3_u128,
		3_usize,
		Int::new(5),
		UInt::new(5),
	);
}
