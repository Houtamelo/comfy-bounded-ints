macro_rules! test_signed_serde {
    ($self: ident) => {
	    let x = $self::<-50, 20>::new(5);
	    let serialized = serde_json::to_string(&x).unwrap();
	    assert_eq!(serialized, "5");
	    let deserialized: $self<-50, 20> = serde_json::from_str(&serialized).unwrap();
	    assert_eq!(deserialized.get(), 5);
	    
	    let x = $self::<-50, 20>::new(-30);
	    let serialized = serde_json::to_string(&x).unwrap();
	    assert_eq!(serialized, "-30");
	    let deserialized: $self<-50, 20> = serde_json::from_str(&serialized).unwrap();
	    assert_eq!(deserialized.get(), -30);
	    
	    let x = $self::<-50, 20>::new(30);
	    let serialized = serde_json::to_string(&x).unwrap();
	    assert_eq!(serialized, "20");
	    let deserialized: $self<-50, 20> = serde_json::from_str(&serialized).unwrap();
	    assert_eq!(deserialized.get(), 20);
	    
	    let x = $self::<-50, 20>::new(-100);
	    let serialized = serde_json::to_string(&x).unwrap();
	    assert_eq!(serialized, "-50");
	    let deserialized: $self<-50, 20> = serde_json::from_str(&serialized).unwrap();
	    assert_eq!(deserialized.get(), -50);
    };
}

macro_rules! test_unsigned_serde {
	($self: ident) => {
	    let x = $self::<10, 50>::new(30);
	    let serialized = serde_json::to_string(&x).unwrap();
	    assert_eq!(serialized, "30");
	    let deserialized: $self<10, 50> = serde_json::from_str(&serialized).unwrap();
	    assert_eq!(deserialized.get(), 30);
	    
	    let x = $self::<10, 50>::new(5);
	    let serialized = serde_json::to_string(&x).unwrap();
	    assert_eq!(serialized, "10");
	    let deserialized: $self<10, 50> = serde_json::from_str(&serialized).unwrap();
	    assert_eq!(deserialized.get(), 10);
	    
	    let x = $self::<10, 50>::new(80);
	    let serialized = serde_json::to_string(&x).unwrap();
	    assert_eq!(serialized, "50");
	    let deserialized: $self<10, 50> = serde_json::from_str(&serialized).unwrap();
	    assert_eq!(deserialized.get(), 50);
	};
}


pub(crate) use test_signed_serde;
pub(crate) use test_unsigned_serde;