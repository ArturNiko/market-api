use uuid::Uuid;

use crate::components::{Item, Order};
use crate::components::order::OrderType;


#[allow(dead_code)]
pub struct Slot {
    item: Item,
    price_base: f32,
    buy_offers: Vec<Order>,
    sell_offers: Vec<Order>,
}

#[allow(dead_code)]
impl Slot {
    pub fn new(item: Item, price_base: f32) -> Self {
        Self {
            item,
            price_base,
            buy_offers: vec![],
            sell_offers: vec![],
        }
    }

    pub fn order(&mut self, order: Order) -> () {
        match order.order_type {
            OrderType::BUY => {
                self.buy_offers.push(order);
                self.buy_offers.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
            }
            OrderType::SELL => {
                self.sell_offers.push(order);
                self.sell_offers.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
            }
        }
    }

    pub fn show(&self) -> () {
        println!("-----------------");
        println!("Item: {}", self.item.name());
        println!("Price base: {}", self.price_base);

        println!("Top buy offers:");
        if self.buy_offers.is_empty() {
            println!("  No buy offers");
        }
        else {
            for (i, order) in self.buy_offers.iter().enumerate().take(5) {
                println!("  {}. Amount: {}, Price: {}", i + 1, order.amount, order.price);
            }
        }

        println!("Top sell offers:");
        if self.sell_offers.is_empty() {
            println!("  No sell offers");
        }
        else {
            for (i, order) in self.sell_offers.iter().enumerate().take(5) {
                println!("  {}. Amount: {}, Price: {}", i + 1, order.amount, order.price);
            }
        }
        println!("-----------------");
    }
    pub fn buy(&mut self, amount: u32) -> Result<(), String> {
        process_action(&mut self.sell_offers, amount, "No sell offers")
    }

    pub fn sell(&mut self, amount: u32) -> Result<(), String> {
        process_action(&mut self.buy_offers, amount, "No buy offers")
    }

    pub fn price_base(&self) -> f32 {
        self.price_base
    }
    pub fn item_name(&self) -> String {
        self.item.name()
    }

    pub fn item_uuid(&self) -> Uuid {
        self.item.uuid()
    }
}

fn process_action(offers: &mut Vec<Order>, amount: u32, no_offers_message: &str) -> Result<(), String> {
    if offers.is_empty() {
        return Err(String::from(no_offers_message));
    }

    let mut amount_left = amount;

    for offer in offers.iter_mut() {
        if amount_left == 0 {
            break;
        }

        if offer.amount > amount_left {
            offer.amount -= amount_left;
            amount_left = 0;
        } else {
            amount_left -= offer.amount;
            offer.amount = 0;
        }
    }

    offers.retain(|offer| offer.amount > 0);

    Ok(())
}