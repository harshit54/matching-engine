pub struct Instrument {
    pub id: i32,
    pub base_asset: String,
    pub quote_asset: String,
}

impl Instrument {
    pub fn symbol(&self) -> String {
        format!("{}-{}", self.base_asset, self.quote_asset)
    }
}
