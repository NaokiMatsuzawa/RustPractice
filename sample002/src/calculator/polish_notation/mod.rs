mod polish_base;

pub fn calc_from_formula(formula: &str) -> Result<i32, String>{
    polish_base::after_change_name::calc_from_formula(formula)
}