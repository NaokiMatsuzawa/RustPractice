mod calculator;

fn main(){
    //let notation = polish_notation::PolishNotation::new("+ + 1 2 3".to_string());
    let ans = calculator::polish_notation::PolishNotation::calc_from_str("+ 1 1909");
    println!("{}", ans.unwrap());
}
