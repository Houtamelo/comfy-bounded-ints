use crate::prelude::{new_bound_signed, new_bound_unsigned};

new_bound_unsigned!(UInt(u64)[u64::MIN, u64::MAX]);
new_bound_signed!(Int(i64)[i64::MIN, i64::MAX]);
