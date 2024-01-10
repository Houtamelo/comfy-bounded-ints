macro_rules! test_div_signed_by_signed {
	($self: ident, $int: ident) => {
	    let mut bound = $self::<-75, 105>::new(100);
		bound /= -5 as $int;
		assert_eq!(bound.get(), -20);
		bound /= 3 as $int;
		assert_eq!(bound.get(), -6);
		bound /= $int::MAX;
		assert_eq!(bound.get(), 0);

		bound.set(100);
	    bound /= $int::MIN;
	    assert_eq!(bound.get(), 0);
	    
	    bound.set(-80);
		bound /= 9 as $int;
		assert_eq!(bound.get(), -8);
	    bound /= -3 as $int;
	    assert_eq!(bound.get(), 2);
	}
}

macro_rules! test_div_signed_by_unsigned {
	($self: ident, $int: ident) => {
	    let mut bound = $self::<-75, 105>::new(100);
		bound /= 5 as $int;
		assert_eq!(bound.get(), 20);
		bound /= 3 as $int;
		assert_eq!(bound.get(), 6);
		bound /= $int::MAX;
		assert_eq!(bound.get(), 0);
	    
	    bound.set(-80);
		bound /= 9 as $int;
		assert_eq!(bound.get(), -8);
	    bound /= 3 as $int;
	    assert_eq!(bound.get(), -2);
	}
}

macro_rules! test_div_unsigned_by_signed {
	($self: ident, $int: ident) => {
	    let mut bound = $self::<0, 105>::new(100);
		bound /= 5 as $int;
		assert_eq!(bound.get(), 20);
		bound /= -3 as $int;
		assert_eq!(bound.get(), 0);
	    
	    bound.set(100);
		bound /= $int::MAX;
		assert_eq!(bound.get(), 0);

		bound.set(80);
	    bound /= $int::MIN;
	    assert_eq!(bound.get(), 0);
	    
	    bound.set(80);
		bound /= -1 as $int;
		assert_eq!(bound.get(), 0);
	}
}

macro_rules! test_div_unsigned_by_unsigned {
	($self: ident, $int: ident) => {
	    let mut bound = $self::<0, 105>::new(100);
		bound /= 5 as $int;
		assert_eq!(bound.get(), 20);
		bound /= 3 as $int;
		assert_eq!(bound.get(), 6);
		bound /= $int::MAX;
		assert_eq!(bound.get(), 0);
	    
	    bound.set(80);
		bound /= 9 as $int;
		assert_eq!(bound.get(), 8);
	    bound /= 3 as $int;
	    assert_eq!(bound.get(), 2);
	}
}

pub(crate) use test_div_signed_by_signed;
pub(crate) use test_div_signed_by_unsigned;
pub(crate) use test_div_unsigned_by_signed;
pub(crate) use test_div_unsigned_by_unsigned;