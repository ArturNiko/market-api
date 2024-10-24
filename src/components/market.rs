use uuid::Uuid;

use crate::components::{Customer, Item, Order};
use crate::components::slot::Slot;

#[allow(dead_code)]
pub struct Market<'a, 'b> {
    name: &'a str,
    market_type: MarketType,
    slots: Vec<Slot<'b>>,
    customers: Vec<Uuid>,
}

#[allow(dead_code)]
impl<'a, 'b> Market<'a, 'b> {
    pub fn new(name: &'a str, market_type: MarketType) -> Self {
        Self {
            name,
            market_type,
            slots: vec![],
            customers: vec![],
        }
    }

    pub fn show_slot(&mut self, item: &Item) -> Result<(), String> {
        if let Some(slot) = self.find_slot(item) {
            slot.show();
            return Ok(());
        }

        Err(String::from("Item is not registered"))
    }

    pub fn buy(&mut self, customer: &Customer, item: &Item, amount: u32) -> Result<(), String> {
        if self.customer_authorized(customer) {
            return Err(String::from("Customer is not authorized"));
        }

        if let Some(slot) = self.find_slot(item) {
            slot.buy(amount)?;
        }

        Ok(())
    }

    pub fn sell(&mut self, customer: &Customer, item: &Item, amount: u32) -> Result<(), String> {
        if self.customer_authorized(customer) {
            return Err(String::from("Customer is not authorized"));
        }

        if let Some(slot) = self.find_slot(item) {
            slot.sell(amount)?;
        }


        Ok(())
    }

    pub fn order(&mut self, customer: &Customer, item: &Item, order: Order) -> Result<(), String> {

        if !self.customer_authorized(customer) {
            return Err(String::from("Customer is not authorized"));
        }

        if let Some(slot) = self.find_slot(item) {

            if order.price < slot.price_base() {
                return Err(String::from("Price is too low"));
            }

            slot.order(order);

            return Ok(());
        }


        Err(String::from("Item is not registered"))
    }

    pub fn register_item(&mut self, item: &Item<'b>, price_base: f32) -> Result<(), String> {
        if price_base <= 1.0 {
            return Err(String::from("Price base must be greater or equal to 1.0"));
        }

        if self.find_slot(item).is_some() {
            return Err(String::from("This item is already registered"));
        }

        let slot = Slot::new(item.clone(), price_base);
        self.slots.push(slot);

        Ok(())
    }

    pub fn unregister_item(&mut self, item: &Item<'b>) -> Result<(), String> {
        if self.remove_slot(item).is_ok() {
            return Ok(());
        }

        Err(String::from("This item is not registered"))
    }

    pub fn authorize_customer(&mut self, customer: &Customer) -> Result<(), String> {
        if self.customer_authorized(customer) {
            return Err(String::from("Customer is already authorized"));
        }

        self.customers.push((*customer).uuid());
        Ok(())
    }

    pub fn deauthorize_customer(&mut self, customer: &Customer) -> Result<(), String> {
        if let Some(index) = self.customers.iter().position(|x| *x == (*customer).uuid()) {
            self.customers.remove(index);
            return Ok(());
        }

        Err(String::from("Customer is not authorized"))
    }

    fn customer_authorized(&self, customer: &Customer) -> bool {
        self.customers.iter().any(|x: &Uuid | *x == customer.uuid())
    }

    fn find_slot(&mut self, item: &Item) -> Option<&mut Slot<'b>> {
        for slot in &mut self.slots {
            if slot.item_uuid() == item.uuid() {
                return Some(slot);
            }
        }

        None
    }

    fn remove_slot(&mut self, item: &Item) -> Result<(), String> {
        if let Some(index) = self.slots.iter().position(|x| x.item_uuid() == item.uuid()) {
            self.slots.remove(index);
            return Ok(());
        }

        Err(String::from("This item is not registered"))
    }
}

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum MarketType {
    CLOSED,
    OPEN,
}
