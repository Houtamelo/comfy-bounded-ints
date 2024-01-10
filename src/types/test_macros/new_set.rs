macro_rules! test_signed_new {
    ($self: ident) => {
	    assert_eq!($self::<-50, 20>::new(5).get(), 5);
		assert_eq!($self::<-50, 20>::new(30).get(), 20);
		assert_eq!($self::<-50, 20>::new(-100).get(), -50);
		assert_eq!($self::<-50, 20>::new(20).get(), 20);
		assert_eq!($self::<-50, 20>::new(-50).get(), -50);
    };
}

macro_rules! test_signed_set {
    ($self: ident) => {
	    let mut bound = $self::<-50, 20>::new(5);
		bound.set(30);
		assert_eq!(bound.get(), 20);
		bound.set(-100);
		assert_eq!(bound.get(), -50);
		bound.set(20);
		assert_eq!(bound.get(), 20);
		bound.set(-50);
		assert_eq!(bound.get(), -50);
    };
}

macro_rules! test_unsigned_new {
    ($self: ident) => {
	    assert_eq!($self::<10, 50>::new(30).get(), 30);
	    assert_eq!($self::<10, 50>::new(5 ).get(), 10);
		assert_eq!($self::<10, 50>::new(80).get(), 50);
    };
}

macro_rules! test_unsigned_set {
	($self: ident) => {
	    let mut bound = $self::<10, 50>::new(5);
		bound.set(30);
		assert_eq!(bound.get(), 30);
		bound.set(80);
		assert_eq!(bound.get(), 50);
		bound.set(5);
		assert_eq!(bound.get(), 10);
	};
}

pub(crate) use test_signed_new;
pub(crate) use test_signed_set;
pub(crate) use test_unsigned_new;
pub(crate) use test_unsigned_set;