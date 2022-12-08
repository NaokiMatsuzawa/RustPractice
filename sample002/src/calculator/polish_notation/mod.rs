enum PolishNotationOperator{
    Add,
    Sub,
}

enum PolishNotationNode{
    Numeric(i32),
    Operator(PolishNotationOperator, Box<PolishNotation>, Box<PolishNotation>),
}

pub struct PolishNotation{
    node : PolishNotationNode,
}

impl PolishNotation{
    pub fn calc_from_str(notation_str: &str) -> i32{
        Self::new(notation_str.to_string()).calc()
    }

    fn new(notation_str: String) -> Self{
        Self::new_sub(&mut notation_str.split_whitespace().rev().collect())
    }

    fn new_sub(notation_str_vec : &mut Vec<&str>) -> Self{
        let str_option = notation_str_vec.pop();
        if str_option == None{
            panic!("invalid input string!");
        }
        let str = str_option.unwrap();
        if let Ok(value) = i32::from_str_radix(str, 10){
            let node = PolishNotationNode::Numeric(value);
            return PolishNotation{node};
        }

        let operator : PolishNotationOperator;
        match str{
            "+" => operator = PolishNotationOperator::Add,
            "-" => operator = PolishNotationOperator::Sub,
            _ => panic!("invalid operator")
        }
        let node = PolishNotationNode::Operator(operator, Box::new(Self::new_sub(notation_str_vec)), Box::new(Self::new_sub(notation_str_vec)));
        PolishNotation{node}
    }

    fn calc(&self)->i32{
        match &self.node{
            PolishNotationNode::Numeric(number) => *number,
            PolishNotationNode::Operator(operator, left, right) =>{
                match operator{
                    PolishNotationOperator::Add => left.calc() + right.calc(),
                    PolishNotationOperator::Sub => left.calc() - right.calc(),
                }
            }
        }
    }
}

#[test]
fn test_numeric_only(){
    assert_eq!(PolishNotation::calc_from_str("1"), 1);
}

#[test]
fn test_simple_add(){
    assert_eq!(PolishNotation::calc_from_str("+ 1 1"), 2);// 1 + 1 = 2
    assert_eq!(PolishNotation::calc_from_str("+ 10 990"), 1000); //10 + 990 = 1000
}

#[test]
fn test_simple_sub(){
    assert_eq!(PolishNotation::calc_from_str("- 1 1"), 0); //1 - 1 = 0
    assert_eq!(PolishNotation::calc_from_str("- 10 100"), -90); //10 - 100 = -90
}
