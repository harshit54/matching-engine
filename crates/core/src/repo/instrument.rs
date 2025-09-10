use crate::repo::error::RepoError;

pub struct InstrumentDbModel {
    pub id: i32,
    pub base_asset: String,
    pub quote_asset: String,
}

pub trait InstrumentRepo {
    fn create_instrument(
        &self,
        base_asset: &str,
        quote_asset: &str,
    ) -> Result<InstrumentDbModel, RepoError>;
    fn get_instrument(&self, id: i32) -> Result<InstrumentDbModel, RepoError>;
    fn list_instruments(&self) -> Result<Vec<InstrumentDbModel>, RepoError>;
}
