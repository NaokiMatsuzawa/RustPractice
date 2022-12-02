mod shape;

fn main() {
    let mut shapes: Vec<Box<dyn shape::Shape>>= Vec::new();
    shapes.push(Box::new(shape::make_square()));
    shapes.push(Box::new(shape::make_rectangle()));
    for shape in &shapes{
        println!("The area of {} is {}", shape.get_name(), shape.area());
    }
}
