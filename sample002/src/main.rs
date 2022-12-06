mod polish_notation;

fn main(){
    let notation = polish_notation::PolishNotation::new("+ + 1 2 3".to_string());
    println!("{}", notation.calc());
}
