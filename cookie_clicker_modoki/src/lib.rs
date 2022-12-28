use std::{collections::HashMap};

use cookie::Cookie;

pub mod cookie;

#[derive(Hash, Clone)]
pub enum AutoProduceComponent{
    Cursor,
    Granma,
    Factory,
}

impl PartialEq for AutoProduceComponent{
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Eq for AutoProduceComponent{}

pub struct CookieProperty{
    cookie : Cookie,
    //auto_components: HashMap<AutoProduceComponent, u32>,
    cursor_num : u32,
    granma_num : u32,
    factory_num : u32,
}

impl CookieProperty{
    pub fn new() -> Self{
        CookieProperty {
             cookie: Cookie::new(0), 
             cursor_num: 0,
             granma_num: 0,
             factory_num: 0,
             //auto_components: HashMap::new(),
        }
    }

    pub fn get_cookie_num(&self) -> u32{
        self.cookie.amount
    }

    pub fn product_cookie_by_click(&mut self){
        self.cookie = self.cookie.add(Cookie::new(1));
    }

    pub fn product_cookie_by_auto(&mut self){
        let add_by_granma = Cookie::new(self.granma_num * get_base_product_num(AutoProduceComponent::Granma));
        let add_by_factory = Cookie::new(self.factory_num * get_base_product_num(AutoProduceComponent::Factory));
        self.cookie = self.cookie.add(add_by_factory);
        self.cookie = self.cookie.add(add_by_granma);
    }

    pub fn add_auto_produce_component(&mut self, component : AutoProduceComponent){
        //self.auto_components.entry(component).and_modify(|num| *num+=1).or_insert(1);
        match component{
            AutoProduceComponent::Cursor => self.cursor_num += 1,
            AutoProduceComponent::Granma => self.granma_num += 1,
            AutoProduceComponent::Factory => self.factory_num +=1,
        }
    }
}

fn get_base_product_num(component: AutoProduceComponent) -> u32{
    match component{
        AutoProduceComponent::Cursor => 0,
        AutoProduceComponent::Granma => 1,
        AutoProduceComponent::Factory => 100,
    }
}