pub mod unit;

use std::collections::HashMap;
use crate::cookie::Cookie;
use crate::AutoProduceComponent;

pub struct CookieProducer{
    auto_components: HashMap<AutoProduceComponent, Box<dyn unit::AutoProducer>>,
}

impl CookieProducer{
    pub fn new() -> Self{
        CookieProducer { 
            auto_components : HashMap::new(),
         }
    }

    pub fn buy_unit(&mut self, wallet : &mut Cookie,  component_label : AutoProduceComponent){
        self.auto_components.entry(component_label.clone()).and_modify(|producer| producer.request_increment_unit_num(wallet)).or_insert(unit::producer_factory(component_label.clone()));

    }

    pub fn calc_produce_cookie_num(&mut self) -> Cookie{
        let mut produce_cookie_num = Cookie::new(0);
        for (_conponent, producer) in &mut self.auto_components{
            produce_cookie_num.add(&producer.get_product_cookie_num());
        }
        produce_cookie_num
    }
}