use crate::prelude::*;

new_generic_bound_unsigned!(Bu8<MIN, MAX>(u8));
new_generic_bound_unsigned!(Bu16<MIN, MAX>(u16));
new_generic_bound_unsigned!(Bu32<MIN, MAX>(u32));
new_generic_bound_unsigned!(Bu64<MIN, MAX>(u64));
new_generic_bound_unsigned!(Bu128<MIN, MAX>(u128));
new_generic_bound_unsigned!(Busize<MIN, MAX>(usize));

new_generic_bound_signed!(Bi8<MIN, MAX>(i8));
new_generic_bound_signed!(Bi16<MIN, MAX>(i16));
new_generic_bound_signed!(Bi32<MIN, MAX>(i32));
new_generic_bound_signed!(Bi64<MIN, MAX>(i64));
new_generic_bound_signed!(Bi128<MIN, MAX>(i128));
new_generic_bound_signed!(Bisize<MIN, MAX>(isize));