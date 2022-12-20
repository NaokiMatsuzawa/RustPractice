pub trait Operator{
    fn new()-> Self where Self : Sized;
    fn calc(&self, left : i32, right : i32)-> i32;
}

pub enum OperatorType{
    Add,
    Sub,
    Error,
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

pub(crate) fn str2operator(str : &str) -> OperatorType{
    match str{
        "+" => OperatorType::Add,
        "-" => OperatorType::Sub,
        _ => OperatorType::Error,
    }
}

pub(crate) fn operator_factory(operator_type: OperatorType) -> Box<dyn Operator>{
    match operator_type{
        OperatorType::Add => Box::new(OperatorAdd::new()),
        OperatorType::Sub => Box::new(OperatorSub::new()),
        OperatorType::Error => panic!("Error operator type is used HERE!"),
    }
}
