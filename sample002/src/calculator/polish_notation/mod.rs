mod polish_notation;
mod operator; // 本当はpolish_notationしか見えなくしたい

pub fn calc_from_formula(formula: &str) -> Result<i32, String>{
    polish_notation::calc_from_formula(formula)
}