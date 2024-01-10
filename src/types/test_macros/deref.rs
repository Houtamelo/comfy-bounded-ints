macro_rules! test_signed_deref {
    ($self: ident) => {
	    assert_eq!(*($self::<-50, 20>::new(5)), 5);
		assert_eq!(*($self::<-100, 50>::new(30)), 30);
		assert_eq!(*($self::<20, 50>::new(-100)), 20);
		assert_eq!(*($self::<-50, -20>::new(20)), -20);
		assert_eq!(*($self::<-50, -20>::new(120)), -20);
		assert_eq!(*($self::<-100, 50>::new(-120)), -100);
    };
}

macro_rules! test_unsigned_deref {
	($self: ident) => {
	    assert_eq!(*($self::<0, 20>::new(5)), 5);
	    assert_eq!(*($self::<10, 50>::new(6)), 10);
	    assert_eq!(*($self::<20, 50>::new(100)), 50);
	    assert_eq!(*($self::<5, 20>::new(25)), 20);
	    assert_eq!(*($self::<8, 20>::new(5)), 8);
	    assert_eq!(*($self::<9, 50>::new(9)), 9);
	};
}

pub(crate) use test_signed_deref;
pub(crate) use test_unsigned_deref;