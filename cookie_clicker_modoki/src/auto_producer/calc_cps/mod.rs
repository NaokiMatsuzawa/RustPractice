use crate::AutoProduceComponent;

use super::CookieProducer;

pub trait CalcCps{
    fn calc_cps(&self, producer : &CookieProducer) -> f64;
}

pub fn cps_calculator_factory(component_label : AutoProduceComponent, base_cps : f64) -> Box<dyn CalcCps>{
    match component_label{
        AutoProduceComponent::Cursor => Box::new(CalcCpsForOnlyBase{base_cps}),
        AutoProduceComponent::Granma => Box::new(CalcCpsForOnlyBase{base_cps}),
        AutoProduceComponent::Factory => Box::new(CalcCpsForFactory{base_cps}),
    }
}

struct CalcCpsForOnlyBase{
    base_cps : f64,
}

impl CalcCps for CalcCpsForOnlyBase{
    fn calc_cps(&self, _producer : &CookieProducer) -> f64 {
        self.base_cps
    }
}

struct CalcCpsForFactory{
    base_cps : f64,
}

impl CalcCps for CalcCpsForFactory{
    fn calc_cps(&self, producer : &CookieProducer) -> f64 {
        let grandma_num = producer.get_units_num(AutoProduceComponent::Granma);
        self.base_cps * (100.0 + grandma_num as f64 / 2.0) / 100.0
    }
}
