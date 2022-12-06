enum PolishNotationOperator{
    Add,
    Sub,
}

enum PolishNotationNode{
    Numeric(i32),
    Operator(PolishNotationOperator),
}

pub struct PolishNotation{
    node : PolishNotationNode,
    left : Box<Option<PolishNotation>>,
    right: Box<Option<PolishNotation>>,
}

impl PolishNotation{
    pub fn new(notation_str: String) -> Self{
        Self::new_sub(&mut notation_str.split_whitespace().rev().collect()).unwrap()
    }

    fn new_sub(notation_str_vec : &mut Vec<&str>) -> Option<Self>{
        let str_option = notation_str_vec.pop();
        if str_option == None{
            return None;
        }
        let str = str_option.unwrap();
        if let Ok(value) = i32::from_str_radix(str, 10){
            let node = PolishNotationNode::Numeric(value);
            return Some(PolishNotation{node, left : Box::new(None), right : Box::new(None)});
        }

        let operator : PolishNotationOperator;
        match str{
            "+" => operator = PolishNotationOperator::Add,
            "-" => operator = PolishNotationOperator::Sub,
            _ => panic!("invalid operator")
        }
        let node = PolishNotationNode::Operator(operator);
        Some(PolishNotation{node, left : Box::new(Self::new_sub(notation_str_vec)), right : Box::new(Self::new_sub(notation_str_vec))})
    }

    pub fn calc(&self)->i32{
        match &self.node{
            PolishNotationNode::Numeric(number) => *number,
            PolishNotationNode::Operator(operator) =>{
                match (self.left.as_ref(), self.right.as_ref()) {
                    (Some(l), Some(r)) =>{
                        match operator{
                            PolishNotationOperator::Add => l.calc() + r.calc(),
                            PolishNotationOperator::Sub => l.calc() - r.calc()
                        }
                    }
                    _ => panic!("invalid")
                }
            }
        }
    }
}

#[test]
fn test_numeric_only(){
    assert_eq!(PolishNotation::new("1".to_string()).calc(), 1);
}

#[test]
fn test_simple_add(){
    assert_eq!(PolishNotation::new("+ 1 1".to_string()).calc(), 2);// 1 + 1 = 2
    assert_eq!(PolishNotation::new("+ 10 990".to_string()).calc(), 1000); //10 + 990 = 1000
}

#[test]
fn test_simple_sub(){
    assert_eq!(PolishNotation::new("- 1 1".to_string()).calc(), 0); //1 - 1 = 0
    assert_eq!(PolishNotation::new("- 10 100".to_string()).calc(), -90); //10 - 100 = -90
}
