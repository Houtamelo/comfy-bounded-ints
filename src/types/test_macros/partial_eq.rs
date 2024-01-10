macro_rules! test_signed_eq {
    ($self: ident) => {
	    assert!($self::<-50, 20>::new(5).eq(&$self::<-50, 20>::new(5)));
		assert!($self::<-50, 20>::new(30).eq(&$self::<-50, 20>::new(50)));
		assert!($self::<-50, 20>::new(-100).eq(&$self::<-50, 20>::new(-80)));
		assert!($self::<-50, 20>::new(20).eq(&$self::<-50, 20>::new(20)));
		assert!($self::<-50, 20>::new(-50).eq(&$self::<-50, 20>::new(-50)));

		assert!($self::<-100, 50>::new(5).eq(&$self::<-50, 20>::new(5)));
		assert!($self::<-100, 50>::new(20).eq(&$self::<-50, 20>::new(20)));
		assert!($self::<-100, 50>::new(-50).eq(&$self::<-50, 20>::new(-100)));
		assert!($self::<-100, 50>::new(-2).eq(&$self::<-50, 20>::new(-2)));
		assert!($self::<-100, 50>::new(20).eq(&$self::<-50, 20>::new(50)));
    };
}

macro_rules! test_signed_ne {
    ($self: ident) => {
	    assert!($self::<-50, 20>::new(5).ne(&$self::<-50, 20>::new(6)));
		assert!($self::<-50, 20>::new(19).ne(&$self::<-50, 20>::new(20)));
		assert!($self::<-50, 20>::new(-50).ne(&$self::<-50, 20>::new(-49)));
		assert!($self::<-50, 20>::new(5).ne(&$self::<-50, 20>::new(-5)));
		assert!($self::<-50, 20>::new(20).ne(&$self::<-50, 20>::new(-50)));

		assert!($self::<-100, 50>::new(5).ne(&$self::<-50, 20>::new(6)));
		assert!($self::<-100, 50>::new(15).ne(&$self::<-50, 20>::new(30)));
		assert!($self::<-100, 50>::new(-50).ne(&$self::<-50, 20>::new(-49)));
		assert!($self::<-100, 50>::new(5).ne(&$self::<-50, 20>::new(-5)));
		assert!($self::<-100, 50>::new(20).ne(&$self::<-50, 20>::new(-50)));
		assert!($self::<-100, 50>::new(-100).ne(&$self::<-50, 20>::new(-100)));
		assert!($self::<-100, 50>::new(50).ne(&$self::<-50, 20>::new(50)));
    };
}

macro_rules! test_unsigned_eq {
    ($self: ident) => {
	    assert!($self::<5, 20>::new(5) .eq(&$self::<5, 20>::new(5 )));
		assert!($self::<5, 20>::new(30).eq(&$self::<5, 20>::new(50)));
		assert!($self::<5, 20>::new(20).eq(&$self::<5, 20>::new(20)));

		assert!($self::<10, 50>::new(5) .eq(&$self::<5, 20>::new(10)));
		assert!($self::<10, 50>::new(20).eq(&$self::<5, 20>::new(20)));
		assert!($self::<10, 50>::new(20).eq(&$self::<5, 20>::new(50)));
    };
}

macro_rules! test_unsigned_ne {
	($self: ident) => {
	    assert!($self::<5, 20>::new(5).ne(&$self::<5, 20>::new(6)));
		assert!($self::<5, 20>::new(19).ne(&$self::<5, 20>::new(20)));
		assert!($self::<5, 20>::new(20).ne(&$self::<5, 20>::new(15)));

		assert!($self::<10, 50>::new(5).ne(&$self::<5, 20>::new(6)));
		assert!($self::<10, 50>::new(15).ne(&$self::<5, 20>::new(30)));
		assert!($self::<10, 50>::new(5).ne(&$self::<5, 20>::new(5)));
		assert!($self::<10, 50>::new(21).ne(&$self::<5, 20>::new(50)));
		assert!($self::<10, 50>::new(50).ne(&$self::<5, 20>::new(50)));
	};
}

pub(crate) use test_signed_eq;
pub(crate) use test_signed_ne;
pub(crate) use test_unsigned_eq;
pub(crate) use test_unsigned_ne;

