use auto_producer::{CookieProducer};
use cookie::*;

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

    pub fn product_cookie_by_auto(&mut self, fps : u64){
        let mut cpf = self.cookie_producer.calc_cps();
        cpf.div_by_u32(fps as u32);
        self.cookie.add(cpf);
    }

    //μs単位
    pub fn calc_duration_to_product_single_cookie(&self) -> u64{
        (1000000.0 / self.cookie_producer.calc_cps().to_f64()) as u64
    }

    pub fn product_single_cookie(&mut self){
        self.cookie.add(Cookie::new(1));
    }

    pub fn add_auto_produce_component(&mut self, component_label : AutoProduceComponent){
        self.cookie_producer.buy_unit(&mut self.cookie, component_label)
    }
}
