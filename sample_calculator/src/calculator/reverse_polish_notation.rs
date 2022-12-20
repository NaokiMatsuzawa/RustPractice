use std::collections::VecDeque;

use super::operator::*;
use super::formula_node::*;

pub fn calc_from_formula(formula: &str) -> Result<i32, String>{
    let binding = formula.to_string();
    let mut parsed_formula : Vec<&str> = binding.split_whitespace().rev().collect();
    formula_factory(&mut parsed_formula).calc()
}

pub(crate) fn formula_factory(formula:&mut Vec<&str>) -> Box<dyn FormulaNode>{
    let mut node_deque = VecDeque::<Box<dyn FormulaNode>>::new();
    while let Some(str) = formula.pop(){
        if let Ok(value) = i32::from_str_radix(str, 10){
            node_deque.push_back(Box::new(FormulaNumericNode::new(value)));
            continue;
        }

        let operator_type = match str{
            "+" => OperatorType::Add,
            "-" => OperatorType::Sub,
            _ => OperatorType::Error,
        };

        match operator_type{
            OperatorType::Error => return Box::new(FormulaErrorNode::new(ErrorType::InvalidCharacters)),
            _ =>{
                if node_deque.len() < 2{
                    return Box::new(FormulaErrorNode::new(ErrorType::FormulaError));
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
        _ => Box::new(FormulaErrorNode::new(ErrorType::FormulaError)),
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