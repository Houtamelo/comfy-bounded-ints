macro_rules! test_signed_from {
    ($self: ident) => {
	    assert_eq!($self::<-50, 20>::from(5).get(), 5);
		assert_eq!($self::<-100, 50>::from(30).get(), 30);
		assert_eq!($self::<20, 50>::from(-100).get(), 20);
		assert_eq!($self::<20, 50>::from(100).get(), 50);
		assert_eq!($self::<-50, -20>::from(20).get(), -20);
		assert_eq!($self::<-50, -20>::from(120).get(), -20);
		assert_eq!($self::<-100, 50>::from(-120).get(), -100);
    };
}

macro_rules! test_unsigned_from {
    ($self: ident) => {
	    assert_eq!($self::<0, 20>::from(5).get(), 5);
	    assert_eq!($self::<10, 50>::from(6).get(), 10);
	    assert_eq!($self::<20, 50>::from(100).get(), 50);
	    assert_eq!($self::<5, 20>::from(25).get(), 20);
	    assert_eq!($self::<8, 20>::from(5).get(), 8);
	    assert_eq!($self::<9, 50>::from(9).get(), 9);
    };
}

pub(crate) use test_signed_from;
pub(crate) use test_unsigned_from;