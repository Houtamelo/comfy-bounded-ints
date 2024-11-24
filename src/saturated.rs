use crate::prelude::{new_bound_signed, new_bound_unsigned};

new_bound_unsigned!(Sat_u8(u8)[u8::MIN, u8::MAX]);
new_bound_unsigned!(Sat_u16(u16)[u16::MIN, u16::MAX]);
new_bound_unsigned!(Sat_u32(u32)[u32::MIN, u32::MAX]);
new_bound_unsigned!(Sat_u64(u64)[u64::MIN, u64::MAX]);
new_bound_unsigned!(Sat_u128(u128)[u128::MIN, u128::MAX]);
new_bound_unsigned!(Sat_usize(usize)[usize::MIN, usize::MAX]);

new_bound_signed!(Sat_i8(i8)[i8::MIN, i8::MAX]);
new_bound_signed!(Sat_i16(i16)[i16::MIN, i16::MAX]);
new_bound_signed!(Sat_i32(i32)[i32::MIN, i32::MAX]);
new_bound_signed!(Sat_i64(i64)[i64::MIN, i64::MAX]);
new_bound_signed!(Sat_i128(i128)[i128::MIN, i128::MAX]);
new_bound_signed!(Sat_isize(isize)[isize::MIN, isize::MAX]);
