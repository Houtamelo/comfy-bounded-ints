use crate::prelude::{new_generic_bound_signed, new_generic_bound_unsigned};

new_generic_bound_unsigned!(Bnd_u8 < MIN, MAX > (u8));
new_generic_bound_unsigned!(Bnd_u16 < MIN, MAX > (u16));
new_generic_bound_unsigned!(Bnd_u32 < MIN, MAX > (u32));
new_generic_bound_unsigned!(Bnd_u64 < MIN, MAX > (u64));
new_generic_bound_unsigned!(Bnd_u128 < MIN, MAX > (u128));
new_generic_bound_unsigned!(Bnd_usize < MIN, MAX > (usize));

new_generic_bound_signed!(Bnd_i8 < MIN, MAX > (i8));
new_generic_bound_signed!(Bnd_i16 < MIN, MAX > (i16));
new_generic_bound_signed!(Bnd_i32 < MIN, MAX > (i32));
new_generic_bound_signed!(Bnd_i64 < MIN, MAX > (i64));
new_generic_bound_signed!(Bnd_i128 < MIN, MAX > (i128));
new_generic_bound_signed!(Bnd_isize < MIN, MAX > (isize));
