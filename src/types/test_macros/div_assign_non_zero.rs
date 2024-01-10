macro_rules! test_signed_div_assign_non_zero {
    ($self: ident, $non_zero: ident) => {
	    let mut bound = $self::<-100, 100>::new(100);
		bound /= $non_zero::new(2).unwrap();
		assert_eq!(bound.get(), 50);
		bound /= $non_zero::new(-10).unwrap();
		assert_eq!(bound.get(), -5);
		bound /= $non_zero::new(50).unwrap();
		assert_eq!(bound.get(), 0);
    };
}

macro_rules! test_unsigned_div_assign_non_zero {
	($self: ident, $non_zero: ident) => {
	    let mut bound = $self::<0, 100>::new(100);
		bound /= $non_zero::new(2).unwrap();
		assert_eq!(bound.get(), 50);
		bound /= $non_zero::new(10).unwrap();
		assert_eq!(bound.get(), 5);
		bound /= $non_zero::new(50).unwrap();
		assert_eq!(bound.get(), 0);
	};
}

pub(crate) use test_signed_div_assign_non_zero;
pub(crate) use test_unsigned_div_assign_non_zero;