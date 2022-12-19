
pub fn calc_from_formula(formula: &str) -> Result<i32, String>{
    todo!();
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
    assert_eq!(calc_from_formula("1 1 +").unwrap(), 2);// 1 + 1 = 2
    assert_eq!(calc_from_formula("10 990 +").unwrap(), 1000); //10 + 990 = 1000
}

#[test]
fn test_simple_sub(){
    assert_eq!(calc_from_formula("1 1 - ").unwrap(), 0); //1 - 1 = 0
    assert_eq!(calc_from_formula("10 100 -").unwrap(), -90); //10 - 100 = -90
}

