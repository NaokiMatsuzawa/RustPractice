use crate::auto_producer::calc_cps::*;

use crate::{AutoProduceComponent};

use super::CookieProducer;

pub trait AutoProducer{
    fn calc_cps(&self, producer : &CookieProducer) -> f64;
    fn request_increment_unit_num(&mut self);
    fn get_units_num(&self) -> u32;
}

pub fn producer_factory(component: AutoProduceComponent) -> Box<dyn AutoProducer>{
    let cps = calc_base_cps(component);
    Box::new(ProducerUnit::new(cps_calculator_factory(component, cps)))
}

fn calc_base_cps(component_id : AutoProduceComponent) -> f64{
    match component_id{
        AutoProduceComponent::Cursor => 0.1,
        AutoProduceComponent::Granma => 1.0,
        AutoProduceComponent::Factory => 100.0,
    }
}

struct ProducerUnit{
    unit_num : u32,
    cps_calculator : Box<dyn CalcCps>,
}

impl ProducerUnit{
    pub fn new(calculator: Box<dyn CalcCps>) -> Self{
        ProducerUnit{
            unit_num : 1,
            cps_calculator: calculator,
        }
    }

    fn calc_cps_per_unit(&self, producer: &CookieProducer) -> f64 {
        self.cps_calculator.calc_cps(producer)
    }   
}

impl AutoProducer for ProducerUnit{
    fn calc_cps(&self, producer: &CookieProducer) -> f64{
        self.calc_cps_per_unit(producer) * self.unit_num as f64
    }

    fn request_increment_unit_num(&mut self) {
        self.unit_num += 1;
    }

    fn get_units_num(&self) -> u32 {
        self.unit_num
    }

}
