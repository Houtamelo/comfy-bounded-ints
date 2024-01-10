macro_rules! test_add_signed_by_signed {
    ($self: ident, $int: ident) => {
	    let mut bound = $self::<-50, 100>::new(50);
		bound += -5 as $int;
		assert_eq!(bound.get(), 45);
		bound += -55 as $int;
		assert_eq!(bound.get(), -10);
		bound += 120 as $int;
		assert_eq!(bound.get(), 100);
		
		bound.set(50);
		bound += $int::MAX;
		assert_eq!(bound.get(), 100);
		
		bound.set(-10);
		bound += $int::MIN;
		assert_eq!(bound.get(), -50);
    };
}

macro_rules! test_add_signed_by_unsigned {
	($self: ident, $int: ident) => {
	    let mut bound = $self::<-50, 100>::new(50);
		bound += 5 as $int;
		assert_eq!(bound.get(), 55);
		bound += 60 as $int;
		assert_eq!(bound.get(), 100);
		
		bound.set(-10);
		bound += $int::MAX;
		assert_eq!(bound.get(), 100);
		
		bound.set(50);
		bound += $int::MIN;
		assert_eq!(bound.get(), 50);
	};
}

macro_rules! test_add_unsigned_by_signed {
	($self: ident, $int: ident) => {
	    let mut bound = $self::<10, 100>::new(50);
		bound += -5 as $int;
		assert_eq!(bound.get(), 45);
		bound += -60 as $int;
		assert_eq!(bound.get(), 10);
		
		bound.set(50);
		bound += $int::MAX;
		assert_eq!(bound.get(), 100);
		
		bound.set(50);
		bound += $int::MIN;
		assert_eq!(bound.get(), 10);
	};
}

macro_rules! test_add_unsigned_by_unsigned {
	($self: ident, $int: ident) => {
	    let mut bound = $self::<10, 100>::new(50);
		bound += 5 as $int;
		assert_eq!(bound.get(), 55);
		bound += 60 as $int;
		assert_eq!(bound.get(), 100);
		
		bound.set(50);
		bound += $int::MAX;
		assert_eq!(bound.get(), 100);
		
		bound.set(50);
		bound += $int::MIN;
		assert_eq!(bound.get(), 50);
	};
}

pub(crate) use test_add_signed_by_signed;
pub(crate) use test_add_signed_by_unsigned;
pub(crate) use test_add_unsigned_by_signed;
pub(crate) use test_add_unsigned_by_unsigned;