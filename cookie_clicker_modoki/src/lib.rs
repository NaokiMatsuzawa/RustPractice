use std::{collections::HashMap};

use cookie::Cookie;

pub mod cookie;

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum AutoProduceComponent{
    Cursor,
    Granma,
    Factory,
}

pub struct CookieProperty{
    cookie : Cookie,
    auto_components: HashMap<AutoProduceComponent, u32>,
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
        self.cookie = self.cookie.add(Cookie::new(1));
    }

    pub fn product_cookie_by_auto(&mut self){
        for (conponent, num) in &self.auto_components{
            let add_cookie = Cookie::new(num * get_base_product_num(conponent.clone()));
            self.cookie = self.cookie.add(add_cookie);
        }
    }

    pub fn add_auto_produce_component(&mut self, component : AutoProduceComponent){
        self.auto_components.entry(component).and_modify(|num| *num+=1).or_insert(1);
    }
}

fn get_base_product_num(component: AutoProduceComponent) -> u32{
    match component{
        AutoProduceComponent::Cursor => 0,
        AutoProduceComponent::Granma => 1,
        AutoProduceComponent::Factory => 100,
    }
}