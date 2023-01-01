use crate::{cookie::Cookie, AutoProduceComponent};

pub trait AutoProducer{
    fn calc_cps(&self) -> f64;
    fn get_product_cookie_num(&mut self) -> Cookie;
    fn request_increment_unit_num(&mut self);
}

pub fn producer_factory(component: AutoProduceComponent) -> Box<dyn AutoProducer>{
    match component{
        AutoProduceComponent::Cursor => Box::new(CursorUnit::new()),
        AutoProduceComponent::Granma => Box::new(GranmaUnit::new()),
        AutoProduceComponent::Factory => Box::new(FactoryUnit::new()),
    }
}


pub struct CursorUnit{
    unit_num : u32,
    cookie_count_as_f64 : f64,
}

impl CursorUnit{
    pub fn new() -> Self{
        CursorUnit{
            unit_num : 1,
            cookie_count_as_f64 : 0.0,
        }
    }
}

impl AutoProducer for CursorUnit{
    fn calc_cps(&self) -> f64 {
        0.1
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

pub struct GranmaUnit{
    unit_num : u32,
    cookie_count_as_f64 : f64,
}

impl GranmaUnit{
    pub fn new() -> Self{
        GranmaUnit{
            unit_num : 1,
            cookie_count_as_f64 : 0.0,
        }
    }
}

impl AutoProducer for GranmaUnit{
    fn calc_cps(&self) -> f64 {
        1.0
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

pub struct FactoryUnit{
    unit_num : u32,
    cookie_count_as_f64 : f64,
}

impl FactoryUnit{
    pub fn new() -> Self{
        FactoryUnit{
            unit_num : 1,
            cookie_count_as_f64 : 0.0,
        }
    }
}

impl AutoProducer for FactoryUnit{
    fn calc_cps(&self) -> f64 {
        100.0
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