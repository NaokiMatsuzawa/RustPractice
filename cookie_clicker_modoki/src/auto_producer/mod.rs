pub mod unit;
mod calc_cps;

use std::collections::HashMap;
use crate::cookie::Cookie;
use crate::AutoProduceComponent;

pub struct CookieProducer{
    auto_components: HashMap<AutoProduceComponent, Box<dyn unit::AutoProducer>>,
    price_table: HashMap<AutoProduceComponent, Cookie>,
}

impl CookieProducer{
    pub fn new() -> Self{
        let mut price_table = HashMap::new();
        let mut insert_table = |id: AutoProduceComponent| {price_table.insert(id, calc_initial_price(id))};

        insert_table(AutoProduceComponent::Cursor);
        insert_table(AutoProduceComponent::Granma);
        insert_table(AutoProduceComponent::Factory);

        CookieProducer { 
            auto_components : HashMap::new(),
            price_table : price_table,
         }
    }

    pub fn buy_unit(&mut self, wallet : &mut Cookie,  component_label : AutoProduceComponent){
        let require_cookies_to_buy = self.price_table.get(&component_label).unwrap();
        if require_cookies_to_buy.is_more(wallet) {
            return;
        }
        wallet.decrease(&require_cookies_to_buy);
        self.price_table.entry(component_label).and_modify(|price| *price = calc_next_cost(price));
        self.auto_components.entry(component_label.clone()).and_modify(|producer| producer.request_increment_unit_num()).or_insert(unit::producer_factory(component_label.clone()));

    }

    pub fn calc_produce_cookie_num(&self) -> Cookie{
        let mut produce_cookie_num = Cookie::new(0);
        for (_conponent, producer) in &self.auto_components{
            let a = producer.get_product_cookie_num(&self);
            produce_cookie_num.add(a);
        }
        produce_cookie_num
    }

    pub fn get_units_num(&self, component_label : AutoProduceComponent) -> u32{
        match self.auto_components.get(&component_label){
            Some(p) =>{
                p.get_units_num()
            },
            None => 0,
        }
    }
}

fn calc_initial_price(component_id: AutoProduceComponent) -> Cookie{
    match component_id{
        AutoProduceComponent::Cursor => Cookie::new(10),
        AutoProduceComponent::Granma => Cookie::new(100),
        AutoProduceComponent::Factory => Cookie::new(1000),
    }
}

fn calc_next_cost(cost : &Cookie) -> Cookie{
    Cookie::new(cost.amount * 115 /100)
}
