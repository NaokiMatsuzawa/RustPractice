enum PolishNotationOperator{
    Add,
    Sub,
}

enum ExceptionType{
    InvalidCharacters,
    FormulaError
}

enum PolishNotationNode{
    Numeric(i32),
    Operator(PolishNotationOperator, Box<PolishNotationOld>, Box<PolishNotationOld>),
    Exception(ExceptionType)
}

pub struct PolishNotationOld{
    node : PolishNotationNode,
}

impl PolishNotationOld{
    pub fn calc_from_formula(formula: &str) -> Result<i32, String>{
        let formula_string = formula.to_string();
        let mut str_vec = formula_string.split_whitespace().rev().collect();
        Self::new(&mut str_vec).calc()
    }

    fn new(notation_str_vec : &mut Vec<&str>) -> Self{
        let str_option = notation_str_vec.pop();
        if str_option == None{
            return PolishNotationOld{node : PolishNotationNode::Exception(ExceptionType::FormulaError)};
        }
        let str = str_option.unwrap();
        if let Ok(value) = i32::from_str_radix(str, 10){
            let node = PolishNotationNode::Numeric(value);
            return PolishNotationOld{node};
        }

        let operator : PolishNotationOperator;
        match str{
            "+" => operator = PolishNotationOperator::Add,
            "-" => operator = PolishNotationOperator::Sub,
            _ => {
                return PolishNotationOld{node : PolishNotationNode::Exception(ExceptionType::InvalidCharacters)};
            }
        }
        let node = PolishNotationNode::Operator(operator, Box::new(Self::new(notation_str_vec)), Box::new(Self::new(notation_str_vec)));
        PolishNotationOld{node}
    }

    fn calc(&self)-> Result<i32, String>{
        match &self.node{
            PolishNotationNode::Numeric(number) => Ok(*number),
            PolishNotationNode::Operator(operator, left, right) =>{
                match (left.calc(), right.calc()){
                    (Ok(left_value), Ok(right_value)) =>{
                        let answer;
                        match operator{
                            PolishNotationOperator::Add => answer = left_value + right_value,
                            PolishNotationOperator::Sub => answer =left_value - right_value,
                        }
                        Ok(answer)
                    }
                    (Err(e), _) => Err(e),
                    (_, Err(e)) => Err(e)
                }
                
            }
            PolishNotationNode::Exception(exception) =>{
                match exception{
                    ExceptionType::FormulaError => Err("Formula Error".to_string()),
                    ExceptionType::InvalidCharacters => Err("Invalid Characters".to_string())
                }
            }
        }
    }
}

#[test]
fn test_invalid_characters(){
    assert_eq!(PolishNotationOld::calc_from_formula("abc").unwrap_err(), "Invalid Characters");
    assert_eq!(PolishNotationOld::calc_from_formula("+ a 1").unwrap_err(), "Invalid Characters");
    assert_eq!(PolishNotationOld::calc_from_formula("+ 1 a").unwrap_err(), "Invalid Characters");
    assert_eq!(PolishNotationOld::calc_from_formula("+ a").unwrap_err(), "Invalid Characters");
    assert_eq!(PolishNotationOld::calc_from_formula("+ a b").unwrap_err(), "Invalid Characters");
}

#[test]
fn test_formula_error(){
    assert_eq!(PolishNotationOld::calc_from_formula("+").unwrap_err(), "Formula Error");
    assert_eq!(PolishNotationOld::calc_from_formula("+ 1").unwrap_err(), "Formula Error");
}

#[test]
fn test_numeric_only(){
    assert_eq!(PolishNotationOld::calc_from_formula("1").unwrap(), 1);
}

#[test]
fn test_simple_add(){
    assert_eq!(PolishNotationOld::calc_from_formula("+ 1 1").unwrap(), 2);// 1 + 1 = 2
    assert_eq!(PolishNotationOld::calc_from_formula("+ 10 990").unwrap(), 1000); //10 + 990 = 1000
}

#[test]
fn test_simple_sub(){
    assert_eq!(PolishNotationOld::calc_from_formula("- 1 1").unwrap(), 0); //1 - 1 = 0
    assert_eq!(PolishNotationOld::calc_from_formula("- 10 100").unwrap(), -90); //10 - 100 = -90
}