#[derive(Clone, Copy)]
pub struct Cookie{
    pub amount: u32,
}

impl Cookie{
    pub fn new(amount : u32) -> Self{
        Cookie{amount}
    }

    pub fn add(&mut self, rhs : Cookie) {
        self.amount += rhs.amount;
    }

    pub fn decrease(&mut self, rhs : &Cookie){
        self.amount -= rhs.amount;
    }

    pub fn is_more(&self, rhs: &Cookie) -> bool{
        self.amount >= rhs.amount
    }
}
