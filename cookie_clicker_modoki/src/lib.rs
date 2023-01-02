use auto_producer::{CookieProducer};
use cookie::Cookie;

pub mod auto_producer;
pub mod cookie;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum AutoProduceComponent{
    Cursor,
    Granma,
    Factory,
}

pub struct CookieProperty{
    cookie : Cookie,
    cookie_producer: CookieProducer,
}

impl CookieProperty{
    pub fn new() -> Self{
        CookieProperty {
             cookie: Cookie::new(0), 
             cookie_producer: CookieProducer::new(),
        }
    }

    pub fn get_cookie_num(&self) -> u32{
        self.cookie.amount
    }

    pub fn product_cookie_by_click(&mut self){
        self.cookie.add(Cookie::new(1));
    }

    pub fn product_cookie_by_auto(&mut self){
        let add_num = Cookie::new(self.cookie_producer.calc_cps() as u32);
        self.cookie.add(add_num);
    }

    //μs単位
    pub fn calc_duration_to_product_single_cookie(&self) -> u64{
        (1000000.0 / self.cookie_producer.calc_cps()) as u64
    }

    pub fn product_single_cookie(&mut self){
        self.cookie.add(Cookie::new(1));
    }

    pub fn add_auto_produce_component(&mut self, component_label : AutoProduceComponent){
        self.cookie_producer.buy_unit(&mut self.cookie, component_label)
    }
}
