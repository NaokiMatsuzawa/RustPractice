use crate::{cookie::Cookie, AutoProduceComponent};

pub trait AutoProducer{
    fn calc_cps(&self) -> f64;
    fn get_product_cookie_num(&mut self) -> Cookie;
    fn request_increment_unit_num(&mut self);
}

pub fn producer_factory(component: AutoProduceComponent) -> Box<dyn AutoProducer>{
    let cps = calc_base_cps(component);
    Box::new(ProducerUnit::new(cps))
}

fn calc_base_cps(component_id : AutoProduceComponent) -> f64{
    match component_id{
        AutoProduceComponent::Cursor => 0.1,
        AutoProduceComponent::Granma => 1.0,
        AutoProduceComponent::Factory => 100.0,
    }
}

pub struct ProducerUnit{
    unit_num : u32,
    base_cps: f64,
    cookie_count_as_f64 : f64,
}

impl ProducerUnit{
    pub fn new(base_cps : f64) -> Self{
        ProducerUnit{
            unit_num : 1,
            base_cps : base_cps,
            cookie_count_as_f64 : 0.0,
        }
    }
}

impl AutoProducer for ProducerUnit{
    fn calc_cps(&self) -> f64 {
        self.base_cps
    }   

    fn get_product_cookie_num(&mut self) -> Cookie {
        self.cookie_count_as_f64 += self.calc_cps() * self.unit_num as f64;
        let cookie_num = if self.cookie_count_as_f64 >= 1.0 {
            let floor = self.cookie_count_as_f64.floor();
            self.cookie_count_as_f64 -= floor;
            floor as u32
        }
        else{
            0
        };
        Cookie::new(cookie_num)
    }

    fn request_increment_unit_num(&mut self) {
        self.unit_num += 1;
    }

}
