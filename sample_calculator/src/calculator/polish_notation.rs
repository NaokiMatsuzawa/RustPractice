use super::operator::*;

pub trait PolishNotation{
    fn calc(&self) -> Result<i32, String>;
}

enum ErrorType{
    InvalidCharacters,
    FormulaError
}
struct PolishError{
    error_type : ErrorType,
}

struct PolishNumeric{
    value : i32
}

struct PolishOperation{
    left : Box<dyn PolishNotation>,
    right: Box<dyn PolishNotation>,
    operator : Box<dyn Operator>
}

impl PolishNotation for PolishNumeric{
    fn calc(&self) -> Result<i32, String> {
        Ok(self.value)
    }
}

impl PolishNotation for PolishError{
    fn calc(&self) -> Result<i32, String> {
        match self.error_type{
            ErrorType::InvalidCharacters => Err("Invalid Characters".to_string()),
            ErrorType::FormulaError => Err("Formula Error".to_string())
        }
    }
}

impl PolishOperation{
    fn new(notation_str_vec : &mut Vec<&str>, operator_type : OperatorType) -> Self where Self : Sized {
        let left = polish_notation_factory(notation_str_vec);
        let right = polish_notation_factory(notation_str_vec);
        let operator = operator_factory(operator_type);
        PolishOperation { left, right , operator}
    }
}

impl PolishNotation for PolishOperation{
    fn calc(&self) -> Result<i32, String> {
        match (self.left.calc(), self.right.calc()){
            (Ok(left_value), Ok(right_value)) => Ok(self.operator.calc(left_value, right_value)),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e)
        }
    }
}


pub fn calc_from_formula(formula: &str) -> Result<i32, String>{
    let formula_string = formula.to_string();
    let mut str_vec = formula_string.split_whitespace().rev().collect();
    polish_notation_factory(&mut str_vec).calc()
}

pub(crate) fn polish_notation_factory(notation_str_vec : &mut Vec<&str>) -> Box<dyn PolishNotation>{
    let str_option = notation_str_vec.pop();
    if str_option == None{
        return Box::new(PolishError{error_type : ErrorType::FormulaError});
    }
    let str = str_option.unwrap();
    if let Ok(value) = i32::from_str_radix(str, 10){
        return Box::new(PolishNumeric{value});
    }

    let operator_type : OperatorType;
    match str{
        "+" => {
            operator_type = OperatorType::Add;
        }
        "-" => {
            operator_type = OperatorType::Sub;
        } 
        _ => {
            return Box::new(PolishError{error_type : ErrorType::InvalidCharacters});
        }
    }
    Box::new(PolishOperation::new(notation_str_vec, operator_type))
}

#[test]
fn test_invalid_characters(){
    assert_eq!(calc_from_formula("abc").unwrap_err(), "Invalid Characters");
    assert_eq!(calc_from_formula("+ a 1").unwrap_err(), "Invalid Characters");
    assert_eq!(calc_from_formula("+ 1 a").unwrap_err(), "Invalid Characters");
    assert_eq!(calc_from_formula("+ a").unwrap_err(), "Invalid Characters");
    assert_eq!(calc_from_formula("+ a b").unwrap_err(), "Invalid Characters");
}

#[test]
fn test_formula_error(){
    assert_eq!(calc_from_formula("+").unwrap_err(), "Formula Error");
    assert_eq!(calc_from_formula("+ 1").unwrap_err(), "Formula Error");
}

#[test]
fn test_numeric_only(){
    assert_eq!(calc_from_formula("1").unwrap(), 1);
}

#[test]
fn test_simple_add(){
    assert_eq!(calc_from_formula("+ 1 1").unwrap(), 2);// 1 + 1 = 2
    assert_eq!(calc_from_formula("+ 10 990").unwrap(), 1000); //10 + 990 = 1000
}

#[test]
fn test_simple_sub(){
    assert_eq!(calc_from_formula("- 1 1").unwrap(), 0); //1 - 1 = 0
    assert_eq!(calc_from_formula("- 10 100").unwrap(), -90); //10 - 100 = -90
}

