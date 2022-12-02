mod shape;

trait Trait<T>{}

use crate::shape::Shape;

fn main() {
    let mut shapes: Vec<Box<dyn shape::Shape>>= Vec::new();
    shapes.push(Box::new(shape::square::Square::new()));
    shapes.push(Box::new(shape::rectangle::Rectangle::new()));
    for shape in &shapes{
        println!("The area of {} is {}", shape.get_name(), shape.area());
    }
}
