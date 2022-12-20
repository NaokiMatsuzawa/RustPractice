use super::operator::*;

pub trait FormulaNode{
    fn calc(&self) -> Result<i32, String>;
}

pub enum ErrorType{
    InvalidCharacters,
    FormulaError
}
pub struct FormulaErrorNode{
    error_type : ErrorType,
}

impl FormulaErrorNode{
    pub fn new(error_type : ErrorType) -> Self{
        FormulaErrorNode { error_type }
    }
}


impl FormulaNode for FormulaErrorNode{
    fn calc(&self) -> Result<i32, String> {
        match self.error_type{
            ErrorType::InvalidCharacters => Err("Invalid Characters".to_string()),
            ErrorType::FormulaError => Err("Formula Error".to_string())
        }
    }
}

pub struct FormulaNumericNode{
    value : i32
}

impl FormulaNumericNode{
    pub fn new(value : i32) -> Self{
        FormulaNumericNode { value }
    }
}

impl FormulaNode for FormulaNumericNode{
    fn calc(&self) -> Result<i32, String> {
        Ok(self.value)
    }
}

pub struct ReversePolishOperation{
    left : Box<dyn FormulaNode>,
    right: Box<dyn FormulaNode>,
    operator : Box<dyn Operator>
}

impl ReversePolishOperation{
    pub fn new(left : Box<dyn FormulaNode>, right: Box<dyn FormulaNode>, operator_type: OperatorType) -> Self {
        ReversePolishOperation{
            left,
            right,
            operator : operator_factory(operator_type),
        }
    }
}

impl FormulaNode for ReversePolishOperation{
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
