mod shape;

fn main() {
    let square = shape::Square{length : 5.0};
    println!("the area of square is {}", square.calc_area());
}
