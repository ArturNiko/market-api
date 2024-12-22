
pub enum ItemList {
    Bitcoin,
    Ethereum,
    Litecoin,
    Dogecoin,
    Cardano,
    Polkadot
}

impl ItemList {
    pub fn as_str(&self) -> &'static str {
        match self {
            ItemList::Bitcoin => "BTC",
            ItemList::Ethereum => "ETH",
            ItemList::Litecoin => "LTC",
            ItemList::Dogecoin => "DOGE",
            ItemList::Cardano => "ADA",
            ItemList::Polkadot => "DOT"
        }
    }
}