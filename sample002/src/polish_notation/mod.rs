enum PolishNotationNode{
    Numeric(i32),
    Operator(char),
    Empty,
}

struct PolishNotation{

}

impl PolishNotation{
    fn new(notation_str: String) -> Self{
        PolishNotation {  }
    }

    fn calc(&self)->i32{
        0
    }
}

#[test]
fn test_simple_add(){
    assert_eq!(PolishNotation::new("+ 1 1".to_string()).calc(), 2);// 1 + 1 = 2
    assert_eq!(PolishNotation::new("+ 10 990".to_string()).calc(), 2); //10 + 990 = 1000
}

#[test]
fn test_simple_sub(){
    assert_eq!(PolishNotation::new("- 1 1".to_string()).calc(), 0); //1 - 1 = 0
    assert_eq!(PolishNotation::new("- 10 100".to_string()).calc(), -90); //10 - 100 = -90
}
