use std::{collections::HashMap};

use auto_producer::unit::AutoProducer;
use cookie::Cookie;

pub mod auto_producer;
pub mod cookie;

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum AutoProduceComponent{
    Cursor,
    Granma,
    Factory,
}

pub struct CookieProperty{
    cookie : Cookie,
    auto_components: HashMap<AutoProduceComponent, Box<dyn AutoProducer>>,
}

impl CookieProperty{
    pub fn new() -> Self{
        CookieProperty {
             cookie: Cookie::new(0), 
             auto_components: HashMap::new(),
        }
    }

    pub fn get_cookie_num(&self) -> u32{
        self.cookie.amount
    }

    pub fn product_cookie_by_click(&mut self){
        self.cookie = self.cookie.add(&Cookie::new(1));
    }

    pub fn product_cookie_by_auto(&mut self){
        for (_conponent, producer) in &mut self.auto_components{
//            let add_cookie = producer.get_product_cookie_num();
            self.cookie = self.cookie.add(&producer.get_product_cookie_num());
        }
    }

    pub fn add_auto_produce_component(&mut self, component : AutoProduceComponent){
        self.auto_components.entry(component.clone()).and_modify(|producer| producer.request_increment_unit_num(&mut self.cookie)).or_insert(auto_producer::unit::producer_factory(component.clone()));
    }
}
