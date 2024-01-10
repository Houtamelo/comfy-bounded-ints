macro_rules! test_signed_default {
    ($self: ident) => {
	    let bound = $self::<-50, 20>::default().get();
	    assert_eq!(bound, -50);
	    let bound = $self::<-100, 50>::default().get();
		assert_eq!(bound, -100);
	    let bound = $self::<20, 50>::default().get();
		assert_eq!(bound, 20);
	    let bound = $self::<-50, -20>::default().get();
		assert_eq!(bound, -50);
    };
}

macro_rules! test_unsigned_default {
    ($self: ident) => {
	    let bound = $self::<10, 20>::default().get();
	    assert_eq!(bound, 10);
	    let bound = $self::<20, 50>::default().get();
		assert_eq!(bound, 20);
	    let bound = $self::<15, 50>::default().get();
		assert_eq!(bound, 15);
	    let bound = $self::<95, 100>::default().get();
		assert_eq!(bound, 95);
    };
}

pub(crate) use test_signed_default;
pub(crate) use test_unsigned_default;