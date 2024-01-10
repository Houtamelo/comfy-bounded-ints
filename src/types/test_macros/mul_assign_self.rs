macro_rules! test_mul_assign_self_signed {
    ($self: ident) => {
	    let mut bound = $self::<-100, 50>::new(5);
		bound *= $self::<-50, 20>::new(5);
		assert_eq!(bound.get(), 25);
		bound *= $self::<-20, 20>::new(100);
		assert_eq!(bound.get(), 50);
		bound *= $self::<-50, 20>::new(-100);
		assert_eq!(bound.get(), -100);
		bound *= $self::<-50, 127>::new(100);
		assert_eq!(bound.get(), -100);
		bound *= $self::<-128, 20>::new(-120);
		assert_eq!(bound.get(), 50);
		bound *= $self::<-50, 20>::new(-1);
		assert_eq!(bound.get(), -50);
		bound *= $self::<-5, 20>::new(0);
		assert_eq!(bound.get(), 0);
    };
}

macro_rules! test_mul_assign_self_unsigned {
	($self: ident) => {
	    let mut bound = $self::<10, 80>::new(5);
		bound *= $self::<5, 20>::new(5);
		assert_eq!(bound.get(), 50);
		bound *= $self::<5, 20>::new(100);
		assert_eq!(bound.get(), 80);
		bound *= $self::<7, 20>::new(100);
		assert_eq!(bound.get(), 80);
	    
	    bound.set(1);
		bound *= $self::<5, 127>::new(3);
		assert_eq!(bound.get(), 50);
		bound *= $self::<1, 20>::new(0);
		assert_eq!(bound.get(), 50);
		bound *= $self::<2, 20>::new(5);
		assert_eq!(bound.get(), 80);
		bound *= $self::<0, 20>::new(0);
		assert_eq!(bound.get(), 10);
	};
}

pub(crate) use test_mul_assign_self_signed;
pub(crate) use test_mul_assign_self_unsigned;