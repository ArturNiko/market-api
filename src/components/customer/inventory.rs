use crate::components::Item;

static MAX_SLOT_SIZE: i8 = 64;
static MAX_INVENTORY_SIZE: i8 = 30;

pub struct Inventory {
    items: Vec<(i32, Item)>,
}

#[allow(dead_code)]
impl Inventory {
    pub fn new(nickname: String) -> Self {
        Self {
            items: vec![],
        }
    }

    pub fn find(&self, item: Vec<&Item>) -> () {

    }

    pub fn remove(&mut self, item: &Item, amount: i32) -> Result<(), String> {
        if let Some((_, i)) = self.find(item) {
            if i.uuid() == item.uuid() {
                return Ok(());
            }
        }

        Err(String::from("Item not found"))
    }

    pub fn add(&mut self, item: Item, amount: i32) -> () {
        self.items.push((amount, item));
    }

    pub fn show(&self) -> () {
        println!("Inventory:");
        for (i, (amount, item)) in self.items.iter().enumerate() {
            println!("  {}. Amount: {}, Item: {}", i + 1, amount, item.name());
        }
    }
}
