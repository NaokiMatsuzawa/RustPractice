use crate::shape::Shape;
pub(crate) struct Square{
    pub length : f64,
    pub name : String,
}

impl Shape for Square{
    fn new() -> Self{
        Square{length : 10.0, name: "square".to_string()}
    }

    fn area(&self) -> f64{
        (self.length * self.length) as f64
    }

    fn get_name(&self) -> &str{
        self.name.as_str()
    }
}

#[test]
fn test_square(){
    assert_eq!(Square{length:5.0, name: "test".to_string()}.area(), 25.0);
    assert_eq!(Square{length:10.0, name: "test".to_string()}.area(), 100.0);
}
