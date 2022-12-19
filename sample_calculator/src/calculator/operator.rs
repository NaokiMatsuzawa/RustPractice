pub trait Operator{
    fn new()-> Self where Self : Sized;
    fn calc(&self, left : i32, right : i32)-> i32;
}

pub enum OperatorType{
    Add,
    Sub,
}

struct OperatorAdd;
struct OperatorSub;

impl Operator for OperatorAdd{
    fn calc(&self, left : i32, right : i32)-> i32{
        left + right
    }

    fn new()-> Self where Self : Sized {
        OperatorAdd{}
    }
}

impl Operator for OperatorSub{
    fn calc(&self, left : i32, right : i32)-> i32{
        left - right
    }

    fn new()-> Self where Self : Sized {
        OperatorSub{}
    }
}


pub(crate) fn operator_factory(operator_type: OperatorType) -> Box<dyn Operator>{
    match operator_type{
        OperatorType::Add => Box::new(OperatorAdd::new()),
        OperatorType::Sub => Box::new(OperatorSub::new())
    }
}
