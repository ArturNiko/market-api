mod components;

use components::{Customer, Item, Market, Order};
use components::{MarketType, OrderType};


fn main() -> Result<(), String> {
    let (mut market, customer, item) = init()?;
    market.show_slot(&item)?;


    Ok(())
}

fn init<'a, 'b>() -> Result<(Market<'a, 'b>, Customer<'a>, Item<'a>), String> {
    let mut market: Market = Market::new(
        "market",
        MarketType::CLOSED,
    );

    let customer: Customer = Customer::new("customer");

    let item = Item::new("vodka");

    market.authorize_customer(&customer)?;
    market.register_item(&item, 20.0)?;

    for i in 0..10 {
        let order = Order::new(
            OrderType::BUY,
            10,
            30.0 + i as f32,
        );

        market.order(&customer, &item, order)?;
    }

    Ok((market, customer, item))
}