macro_rules! test_sub_assign_self_signed {
    ($self: ident) => {
		let mut bound = $self::<-100, 50>::new(5);
		bound -= $self::<-50, 20>::new(5);
		assert_eq!(bound.get(), 0);
		bound -= $self::<-20, 20>::new(100);
		assert_eq!(bound.get(), -20);
		bound -= $self::<-50, 20>::new(-100);
		assert_eq!(bound.get(), 30);
		bound -= $self::<-50, 127>::new(100);
		assert_eq!(bound.get(), -70);
		bound -= $self::<-128, 20>::new(-120);
		assert_eq!(bound.get(), 50);
		
		bound.set(20);
		bound -= $self::<-50, 20>::new(-50);
		assert_eq!(bound.get(), 50);
    };
}

macro_rules! test_sub_assign_self_unsigned {
	($self: ident) => {
		let mut bound = $self::<10, 50>::new(20);
		bound -= $self::<5, 20>::new(5);
		assert_eq!(bound.get(), 15);
		bound -= $self::<5, 20>::new(100);
		assert_eq!(bound.get(), 10);
		bound -= $self::<30, 127>::new(100);
		assert_eq!(bound.get(), 10);
		
		bound.set(100);
		bound -= $self::<10, 20>::new(30);
		assert_eq!(bound.get(), 30);
		bound -= $self::<10, 20>::new(5);
		assert_eq!(bound.get(), 20);
	};
}

pub(crate) use test_sub_assign_self_signed;
pub(crate) use test_sub_assign_self_unsigned;