macro_rules! test_signed_add_assign_self {
    ($self: ident) => {
	    let mut bound = $self::<-50, 80>::new(5);
		bound += $self::<-10, 20>::new(5);
		assert_eq!(bound.get(), 10);
		bound += $self::<-15, 23>::new(100);
		assert_eq!(bound.get(), 33);
		bound += $self::<-80, 100>::new(-100);
		assert_eq!(bound.get(), -47);
		bound += $self::<-50, 20>::new(100);
		assert_eq!(bound.get(), -27);
		bound += $self::<-45, 29>::new(-100);
		assert_eq!(bound.get(), -50);
		bound += $self::<5, 80>::new(-50);
		assert_eq!(bound.get(), -45);
    };
}

macro_rules! test_unsigned_add_assign_self {
	($self: ident) => {
		let mut bound = $self::<5, 100>::new(5);
		bound += $self::<10, 20>::new(5);
		assert_eq!(bound.get(), 15);
		bound += $self::<15, 23>::new(100);
		assert_eq!(bound.get(), 38);
		bound += $self::<80, 100>::new(100);
		assert_eq!(bound.get(), 100);
		
		bound.set(0);
		bound += $self::<10, 20>::new(100);
		assert_eq!(bound.get(), 25);
		bound += $self::<25, 29>::new(0);
		assert_eq!(bound.get(), 50);
		bound += $self::<5, 80>::new(45);
		assert_eq!(bound.get(), 95);
	};
}

pub(crate) use test_signed_add_assign_self;
pub(crate) use test_unsigned_add_assign_self;