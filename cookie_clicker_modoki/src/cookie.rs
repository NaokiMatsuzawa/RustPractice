#[derive(Clone, Copy)]
pub struct Cookie{
    pub amount: u32,
    cookie_piece: f64,
}

impl Cookie{
    pub fn new(amount : u32) -> Self{
        Cookie{amount, cookie_piece : 0.0}
    }

    pub fn add(&mut self, rhs : Cookie) {
        self.amount += rhs.amount;
        self.cookie_piece += rhs.cookie_piece;
        if self.cookie_piece > 1.0{
            let add_value = self.cookie_piece.floor();
            self.amount += add_value as u32;
            self.cookie_piece -= add_value;
        }
    }

    pub fn decrease(&mut self, rhs : &Cookie){
        //rhsのcookie_pieceは0のみを想定
        //ただ0以外でも特に大きな問題はないのでそのまま
        self.amount -= rhs.amount;
    }

    pub fn is_more(&self, rhs: &Cookie) -> bool{
        //rhsのcookie_pieceは0のみを想定
        //ただ0以外でも特に大きな問題はないのでそのまま
        self.amount >= rhs.amount
    }

    pub fn div_by_u32(&mut self, rhs: u32){
        self.cookie_piece += (self.amount % rhs) as f64;
        self.amount /= rhs;
        self.cookie_piece /= rhs as f64;
    }

    pub fn to_f64(&self) -> f64{
        let mut value = self.cookie_piece;
        value += self.amount as f64;
        value
    }
}

pub fn f64_to_cookie(value: f64) -> Cookie{
    assert!(value >= 0.0);
    let amount = value.floor() as u32;
    let cookie_piece = value - amount as f64;//valueの小数部を取得する関数があればそっちを使う
    Cookie { amount, cookie_piece }
}

