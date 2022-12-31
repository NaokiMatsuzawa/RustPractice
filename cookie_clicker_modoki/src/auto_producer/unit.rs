use crate::{cookie::Cookie, AutoProduceComponent};

pub trait AutoProducer{
    fn calc_cps(&self) -> f64;
    fn get_product_cookie_num(&mut self) -> Cookie;
    fn request_increment_unit_num(&mut self, wallet : &mut Cookie);
}

pub fn producer_factory(component: AutoProduceComponent) -> Box<dyn AutoProducer>{
    match component{
        AutoProduceComponent::Cursor => Box::new(CursorUnit::new()),
        AutoProduceComponent::Granma => Box::new(GranmaUnit::new()),
        AutoProduceComponent::Factory => Box::new(FactoryUnit::new()),
    }
}

fn calc_next_cost(cost : &Cookie) -> Cookie{
    Cookie::new(cost.amount * 1.15 as u32)
}

pub struct CursorUnit{
    unit_num : u32,
    cookie_count_as_f64 : f64,
    require_cookies_to_buy : Cookie,
}

impl CursorUnit{
    pub fn new() -> Self{
        CursorUnit{
            unit_num : 0,
            cookie_count_as_f64 : 0.0,
            require_cookies_to_buy : Cookie::new(10),
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

    fn request_increment_unit_num(&mut self, wallet : &mut Cookie) {
        if self.require_cookies_to_buy.is_more(wallet) {
            return;
        }
        wallet.decrease(&self.require_cookies_to_buy);
        self.require_cookies_to_buy = calc_next_cost(&self.require_cookies_to_buy);
        self.unit_num += 1;
    }

}

pub struct GranmaUnit{
    unit_num : u32,
    cookie_count_as_f64 : f64,
    require_cookies_to_buy : Cookie,
}

impl GranmaUnit{
    pub fn new() -> Self{
        GranmaUnit{
            unit_num : 0,
            cookie_count_as_f64 : 0.0,
            require_cookies_to_buy : Cookie::new(100),
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

    fn request_increment_unit_num(&mut self, wallet : &mut Cookie) {
        if self.require_cookies_to_buy.is_more(wallet){
            return;
        }
        wallet.decrease(&self.require_cookies_to_buy);
        self.require_cookies_to_buy = calc_next_cost(&self.require_cookies_to_buy);
        self.unit_num += 1;
    }

}

pub struct FactoryUnit{
    unit_num : u32,
    cookie_count_as_f64 : f64,
    require_cookies_to_buy : Cookie,
}

impl FactoryUnit{
    pub fn new() -> Self{
        FactoryUnit{
            unit_num : 0,
            cookie_count_as_f64 : 0.0,
            require_cookies_to_buy : Cookie::new(1000),
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

    fn request_increment_unit_num(&mut self, wallet : &mut Cookie) {
        if self.require_cookies_to_buy.is_more(wallet){
            return;
        }
        wallet.decrease(&self.require_cookies_to_buy);
        self.require_cookies_to_buy = calc_next_cost(&self.require_cookies_to_buy);
        self.unit_num += 1;
    }

}