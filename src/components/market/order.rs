#[allow(dead_code)]
#[derive(Debug)]
pub struct Order {
    pub amount: u32,
    pub price: f32,
    pub order_type: OrderType
}

#[allow(dead_code)]
impl Order {
    pub fn new(order_type: OrderType, amount: u32, price: f32) -> Self {
        Self {
            amount,
            price,
            order_type
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum OrderType {
    SELL,
    BUY
}