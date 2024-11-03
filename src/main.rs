mod components;

use components::{Customer, Item, Market, Order};
use components::{MarketType, OrderType};


fn main() -> Result<(), String> {
    let (mut market, customer, item) = init()?;

    println!("Nickname: {}", customer.nickname());
    market.show_slot(&item)?;

    market.buy(&customer, &item, 14)?;

    market.show_slot(&item)?;

    Ok(())
}

fn init() -> Result<(Market, Customer, Item), String> {
    let mut market: Market = Market::new(
        String::from("market"),
        MarketType::CLOSED,
    );

    let customer: Customer = Customer::new(String::from("customer"));

    let item = Item::new(String::from("vodka"));

    market.authorize_customer(&customer)?;
    market.register_item(&item, 20.0)?;

    for i in 0..10 {
        let order = Order::new(
            OrderType::SELL,
            10,
            30.0 + i as f32,
        );

        market.order(&customer, &item, order)?;
    }

    Ok((market, customer, item))
}