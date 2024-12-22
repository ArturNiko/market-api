mod components;
mod app;

use components::{Customer, Item, Market, Order};
use components::{MarketType, OrderType};
use app::*;

fn main() -> Result<(), tokio_postgres::Error> {
    main2()?;

    /**
    let (mut market, customer, item) = init()?;

    println!("Nickname: {}", customer.nickname());
    market.show_slot(&item)?;

    market.buy(&customer, &item, 164)?;

    market.show_slot(&item)?;

    market.sell(&customer, &item, 10)?;
    **/
    Ok(())
}

fn init() -> Result<(Market, Customer, Item), String> {
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
            OrderType::SELL,
            10,
            30.0 + i as f32,
        );

        market.order(&customer, &item, order)?;
    }

    Ok((market, customer, item))
}