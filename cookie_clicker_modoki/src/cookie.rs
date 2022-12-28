pub struct Cookie{
    pub amount: u32,
}

impl Cookie{
    pub fn new(amount : u32) -> Self{
        Cookie{amount}
    }

    pub fn add(&self, other : Cookie) -> Self{
        Cookie{amount: self.amount + other.amount}
    }

    pub fn decrease(&self, other : Cookie) -> Self{
        Cookie{amount: self.amount - other.amount}
    }
}
