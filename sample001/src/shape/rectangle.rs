use crate::shape::Shape;

pub struct Rectangle{
    width: f64,
    height: f64,
}

impl Shape for Rectangle{
    fn new() -> Rectangle{
        Rectangle{width: 5.0, height :4.0}
    }

    fn area(&self)->f64{
        self.width * self.height
    }

    fn get_name(&self) -> &str {
        "rectangle"
    }
}

#[test]
fn test_rectangle(){
    assert_eq!(Rectangle{width:10.0, height : 3.0}.area(), 30.0);
}