
pub enum CurrencyList {
    Bitcoin,
    Ethereum,
    Litecoin,
    Dogecoin,
    Cardano,
    Polkadot
}

impl CurrencyList {
    pub fn as_str(&self) -> &'static str {
        match self {
            CurrencyList::Bitcoin => "BTC",
            CurrencyList::Ethereum => "ETH",
            CurrencyList::Litecoin => "LTC",
            CurrencyList::Dogecoin => "DOGE",
            CurrencyList::Cardano => "ADA",
            CurrencyList::Polkadot => "DOT"
        }
    }
}