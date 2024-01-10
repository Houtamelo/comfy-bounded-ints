macro_rules! test_mul_signed_by_signed {
    ($self: ident, $int: ident) => {
	    let mut bound = $self::<-60, 90>::new(5);
		bound *= -5 as $int;
		assert_eq!(bound.get(), -25);
		bound *= 2 as $int;
		assert_eq!(bound.get(), -50);
	    bound *= -1 as $int;
	    assert_eq!(bound.get(), 50);
	    
		bound *= $int::MAX;
		assert_eq!(bound.get(), 90);
		bound *= $int::MIN;
		assert_eq!(bound.get(), -60);

		bound *= 0 as $int;
		assert_eq!(bound.get(), 0);
    };
}

macro_rules! test_mul_signed_by_unsigned {
	($self: ident, $int: ident) => {
	    let mut bound = $self::<-60, 90>::new(5);
		bound *= 5 as $int;
		assert_eq!(bound.get(), 25);
		bound *= 1 as $int;
		assert_eq!(bound.get(), 25);
	    
	    bound *= $int::MAX;
	    assert_eq!(bound.get(), 90);
	    bound *= $int::MIN;
	    assert_eq!(bound.get(), 0);

		bound.set(-2);
		bound *= 2 as $int;
		assert_eq!(bound.get(), -4);
		bound *= 1 as $int;
		assert_eq!(bound.get(), -4);
	    
	    bound *= $int::MAX;
	    assert_eq!(bound.get(), -60);
	    bound *= $int::MIN;
	    assert_eq!(bound.get(), 0);
	};
}

macro_rules! test_mul_unsigned_by_signed {
	($self: ident, $int: ident) => {
	    let mut bound = $self::<10, 90>::new(20);
		bound *= -5 as $int;
		assert_eq!(bound.get(), 10);
		bound *= 2 as $int;
		assert_eq!(bound.get(), 20);
	    bound *= -1 as $int;
	    assert_eq!(bound.get(), 10);
	    
		bound *= $int::MAX;
		assert_eq!(bound.get(), 90);
		bound *= $int::MIN;
		assert_eq!(bound.get(), 10);

		bound *= 0 as $int;
		assert_eq!(bound.get(), 10);
	};
}

macro_rules! test_mul_unsigned_by_unsigned {
	($self: ident, $int: ident) => {
	    let mut bound = $self::<10, 90>::new(20);
		bound *= 3 as $int;
		assert_eq!(bound.get(), 60);
		bound *= 1 as $int;
		assert_eq!(bound.get(), 60);
	    
	    bound *= $int::MIN;
	    assert_eq!(bound.get(), 10);
	    bound *= $int::MAX;
	    assert_eq!(bound.get(), 90);
	};
}

pub(crate) use test_mul_signed_by_signed;
pub(crate) use test_mul_signed_by_unsigned;
pub(crate) use test_mul_unsigned_by_signed;
pub(crate) use test_mul_unsigned_by_unsigned;