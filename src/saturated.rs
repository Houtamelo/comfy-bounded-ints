use crate::prelude::*;

new_bound_unsigned!(Satu8(u8)[u8::MIN, u8::MAX]);
new_bound_unsigned!(Satu16(u16)[u16::MIN, u16::MAX]);
new_bound_unsigned!(Satu32(u32)[u32::MIN, u32::MAX]);
new_bound_unsigned!(Satu64(u64)[u64::MIN, u64::MAX]);
new_bound_unsigned!(Satu128(u128)[u128::MIN, u128::MAX]);
new_bound_unsigned!(Satusize(usize)[usize::MIN, usize::MAX]);

new_bound_signed!(Sati8(i8)[i8::MIN, i8::MAX]);
new_bound_signed!(Sati16(i16)[i16::MIN, i16::MAX]);
new_bound_signed!(Sati32(i32)[i32::MIN, i32::MAX]);
new_bound_signed!(Sati64(i64)[i64::MIN, i64::MAX]);
new_bound_signed!(Sati128(i128)[i128::MIN, i128::MAX]);
new_bound_signed!(Satisize(isize)[isize::MIN, isize::MAX]);