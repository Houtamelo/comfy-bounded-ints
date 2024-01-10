macro_rules! test_rem_signed_by_signed {
    ($self: ident, $int: ident) => {
		let mut bound = $self::<-80, 105>::new(100);
		bound %= -6 as $int;
		assert_eq!(bound.get(), 4);
		bound %= 3 as $int;
		assert_eq!(bound.get(), 1);
		bound %= $int::MAX;
		assert_eq!(bound.get(), 1);
		bound %= $int::MIN;
		assert_eq!(bound.get(), 1);
		bound %= 1 as $int;
		assert_eq!(bound.get(), 0);
		
		bound.set(-80);
		bound %= -9 as $int;
		assert_eq!(bound.get(), -8);
		bound %= 3 as $int;
		assert_eq!(bound.get(), -2);
		bound %= $int::MAX;
		assert_eq!(bound.get(), -2);
		bound %= $int::MIN;
		assert_eq!(bound.get(), -2);
		bound %= 1 as $int;
		assert_eq!(bound.get(), 0);
    }
}

macro_rules! test_rem_signed_by_unsigned {
    ($self: ident, $int: ident) => {
		let mut bound = $self::<-80, 105>::new(100);
		bound %= 6 as $int;
		assert_eq!(bound.get(), 4);
		bound %= 3 as $int;
		assert_eq!(bound.get(), 1);
		bound %= $int::MAX;
		assert_eq!(bound.get(), 1);
		bound %= 1 as $int;
		assert_eq!(bound.get(), 0);
		
		bound.set(-80);
		bound %= 9 as $int;
		assert_eq!(bound.get(), -8);
		bound %= 3 as $int;
		assert_eq!(bound.get(), -2);
		bound %= $int::MAX;
		assert_eq!(bound.get(), -2);
		bound %= 1 as $int;
		assert_eq!(bound.get(), 0);
    }
}

macro_rules! test_rem_unsigned_by_signed {
	($self: ident, $int: ident) => {
		let mut bound = $self::<0, 105>::new(100);
		bound %= -6 as $int;
		assert_eq!(bound.get(), 4);
		bound %= 3 as $int;
		assert_eq!(bound.get(), 1);
		bound %= $int::MAX;
		assert_eq!(bound.get(), 1);
		bound %= 1 as $int;
		assert_eq!(bound.get(), 0);
	}
}

macro_rules! test_rem_unsigned_by_unsigned {
	($self: ident, $int: ident) => {
		let mut bound = $self::<0, 105>::new(100);
		bound %= 6 as $int;
		assert_eq!(bound.get(), 4);
		bound %= 3 as $int;
		assert_eq!(bound.get(), 1);
		bound %= $int::MAX;
		assert_eq!(bound.get(), 1);
		bound %= 1 as $int;
		assert_eq!(bound.get(), 0);
	}
}

pub(crate) use test_rem_signed_by_signed;
pub(crate) use test_rem_signed_by_unsigned;
pub(crate) use test_rem_unsigned_by_signed;
pub(crate) use test_rem_unsigned_by_unsigned;