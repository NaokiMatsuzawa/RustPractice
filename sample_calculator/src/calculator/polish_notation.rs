use super::operator::*;
use super::formula_node::*;

pub fn calc_from_formula(formula: &str) -> Result<i32, String>{
    let formula_string = formula.to_string();
    let mut str_vec = formula_string.split_whitespace().rev().collect();
    polish_notation_factory(&mut str_vec).calc()
}

pub(crate) fn polish_notation_factory(notation_str_vec : &mut Vec<&str>) -> Box<dyn FormulaNode>{
    let str_option = notation_str_vec.pop();
    if str_option == None{
        return Box::new(FormulaErrorNode::new(ErrorType::FormulaError));
    }
    let str = str_option.unwrap();
    if let Ok(value) = i32::from_str_radix(str, 10){
        return Box::new(FormulaNumericNode::new(value));
    }

    let operator_type = str2operator(&str);
    match operator_type{
        OperatorType::Error => return Box::new(FormulaErrorNode::new(ErrorType::InvalidCharacters)),
        _ =>{
            let left_node = polish_notation_factory(notation_str_vec);
            let right_node = polish_notation_factory(notation_str_vec);
            Box::new(FormulaOperationNode::new(left_node, right_node, operator_type))
        }
    }
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

#[test]
fn test_simple_mul(){
    assert_eq!(calc_from_formula("* 2 -10").unwrap(), -20); // 2 * -10
    assert_eq!(calc_from_formula("* -5 -20").unwrap(), 100); // -5 * -20
}

#[test]
fn test_complex(){
    assert_eq!(calc_from_formula("+ - 1 2 3").unwrap(), 2);// (1-2)+3
    assert_eq!(calc_from_formula("- + 1 2 3").unwrap(), 0);// (1+2)-3
    assert_eq!(calc_from_formula("+ 1 - 2 3").unwrap(), 0);// (1+(2-3))
    assert_eq!(calc_from_formula("- 1 + 2 3").unwrap(), -4);// (1-(2+3))
}