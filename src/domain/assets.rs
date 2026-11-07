struct Asset{
    id: u32,
    ticker: String,
    name: String,
    decimals: u8
}

impl Asset{
    pub fn new(id:u32, ticker:String, name:String, decimals:u8) -> Self {
        Self { id, ticker, name, decimals }
    }   

    pub fn id(&self) -> &u32 {
        &self.id
    }
    pub fn ticker(&self) -> &str {
        &self.ticker
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn decimals(&self) -> u8 {
        self.decimals
    }
}

#[cfg(test)]
mod tests {
    use super::Asset;
    #[test]
    fn test_asset_new() {
        let asset = Asset::new(1, String::from("BTC"), String::from("Bitcoin"), 8);
        assert_eq!(asset.id(), &1);
        assert_eq!(asset.ticker(), "BTC");
        assert_eq!(asset.name(), "Bitcoin");
        assert_ne!(asset.ticker(), "USD");
        assert_eq!(asset.decimals(), 8);
        assert_ne!(asset.decimals(), 9);
    }
}