use crate::shape::Shape;
pub(crate) struct Square{
    pub length : f64,
}

impl Shape for Square{
    fn new() -> Self{
        Square{length : 10.0}
    }

    fn area(&self) -> f64{
        (self.length * self.length) as f64
    }
}

#[test]
fn test_square(){
    assert_eq!(Square{length:5.0}.area(), 25.0);
    assert_eq!(Square{length:10.0}.area(), 100.0);
}
