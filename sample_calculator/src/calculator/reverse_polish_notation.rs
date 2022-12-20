use std::collections::VecDeque;

use super::operator::*;

pub trait ReversePolishFormulaNode{
    fn calc(&self) -> Result<i32, String>;
}

enum ReversePolishErrorType{
    InvalidCharacters,
    FormulaError,
}
struct ReversePolishError{
    error: ReversePolishErrorType,
}

impl ReversePolishError{
    fn new(error: ReversePolishErrorType) -> Self{
        ReversePolishError { error }
    }
}

impl ReversePolishFormulaNode for ReversePolishError{
    fn calc(&self) -> Result<i32, String> {
        match self.error{
            ReversePolishErrorType::FormulaError => Err("Formula Error".to_string()),
            ReversePolishErrorType::InvalidCharacters => Err("Invalid Characters".to_string()),
        }
    }
}

struct ReversePolishNumericNode{
    value : i32,
}

impl ReversePolishNumericNode{
    fn new(value : i32) -> Self{
        ReversePolishNumericNode { value }
    }
}

impl ReversePolishFormulaNode for ReversePolishNumericNode{
    fn calc(&self) -> Result<i32, String>{
        Ok(self.value)
    }
}

struct ReversePolishOperation{
    left : Box<dyn ReversePolishFormulaNode>,
    right: Box<dyn ReversePolishFormulaNode>,
    operator : Box<dyn Operator>
}

impl ReversePolishOperation{
    fn new(left : Box<dyn ReversePolishFormulaNode>, right: Box<dyn ReversePolishFormulaNode>, operator_type: OperatorType) -> Self {
        ReversePolishOperation{
            left,
            right,
            operator : operator_factory(operator_type),
        }
    }
}

impl ReversePolishFormulaNode for ReversePolishOperation{
    fn calc(&self) -> Result<i32, String> {
        let left_result = self.left.calc();
        let right_result = self.right.calc();
        match (left_result, right_result) {
            (Err(str), _) => Err(str),
            (_, Err(str)) => Err(str),
            (Ok(left_value), Ok(right_value)) => Ok(self.operator.calc(left_value, right_value)),
        }
    }
}

pub fn calc_from_formula(formula: &str) -> Result<i32, String>{
    let binding = formula.to_string();
    let mut parsed_formula : Vec<&str> = binding.split_whitespace().rev().collect();
    formula_factory(&mut parsed_formula).calc()
}

pub(crate) fn formula_factory(formula:&mut Vec<&str>) -> Box<dyn ReversePolishFormulaNode>{
    let mut node_deque = VecDeque::<Box<dyn ReversePolishFormulaNode>>::new();
    while let Some(str) = formula.pop(){
        if let Ok(value) = i32::from_str_radix(str, 10){
            node_deque.push_back(Box::new(ReversePolishNumericNode::new(value)));
            continue;
        }

        let operator_type = match str{
            "+" => OperatorType::Add,
            "-" => OperatorType::Sub,
            _ => OperatorType::Error,
        };

        match operator_type{
            OperatorType::Error => return Box::new(ReversePolishError::new(ReversePolishErrorType::InvalidCharacters)),
            _ =>{
                if node_deque.len() < 2{
                    return Box::new(ReversePolishError::new(ReversePolishErrorType::FormulaError));
                }
                let right = node_deque.pop_back().unwrap();
                let left = node_deque.pop_back().unwrap();
                let new_node = Box::new(ReversePolishOperation::new(left, right, operator_type));
                node_deque.push_back(new_node);
                continue;
            }
        }
    }
    match node_deque.len() {
        1 => node_deque.pop_front().unwrap(),
        _ => Box::new(ReversePolishError::new(ReversePolishErrorType::FormulaError)),
    }
}


#[test]
fn test_invalid_characters(){
    assert_eq!(calc_from_formula("abc").unwrap_err(), "Invalid Characters");
    assert_eq!(calc_from_formula("a 1 +").unwrap_err(), "Invalid Characters");
    assert_eq!(calc_from_formula("1 a +").unwrap_err(), "Invalid Characters");
    assert_eq!(calc_from_formula("a +").unwrap_err(), "Invalid Characters");
    assert_eq!(calc_from_formula("a b +").unwrap_err(), "Invalid Characters");
}

#[test]
fn test_formula_error(){
    assert_eq!(calc_from_formula("+").unwrap_err(), "Formula Error");
    assert_eq!(calc_from_formula("+ 1").unwrap_err(), "Formula Error");
    assert_eq!(calc_from_formula("1 +").unwrap_err(), "Formula Error");
    assert_eq!(calc_from_formula("1 1 + 1").unwrap_err(), "Formula Error");
    assert_eq!(calc_from_formula("1 1 1 +").unwrap_err(), "Formula Error");
}

#[test]
fn test_numeric_only(){
    assert_eq!(calc_from_formula("1").unwrap(), 1);
}

#[test]
fn test_simple_add(){
    assert_eq!(calc_from_formula("1 1 +").unwrap(), 2);// 1 + 1 = 2
    assert_eq!(calc_from_formula("10 990 +").unwrap(), 1000); //10 + 990 = 1000
}

#[test]
fn test_simple_sub(){
    assert_eq!(calc_from_formula("1 1 - ").unwrap(), 0); //1 - 1 = 0
    assert_eq!(calc_from_formula("10 100 -").unwrap(), -90); //10 - 100 = -90
}

#[test]
fn test_complex(){
    assert_eq!(calc_from_formula("1 1 + 3 -").unwrap(), -1); // 1 + 1 - 3
    assert_eq!(calc_from_formula("1 1 3 + -").unwrap(), -3); // 1 - (1 + 3)
    
}