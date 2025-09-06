use editor_tools::prelude::AssetArmySlotModel;
use sheets_connector::{prelude::ValueRange, utils::SheetsValueRangeConverter};
use uuid::Uuid;

pub struct SheetToArmyAssetsConverter {
    asset_id: Uuid
}

impl SheetToArmyAssetsConverter {
    pub fn new(asset_id: Uuid) -> Self {
        SheetToArmyAssetsConverter { asset_id }
    }
}

impl SheetsValueRangeConverter for SheetToArmyAssetsConverter {
    type Output = Vec<AssetArmySlotModel>;
    
    fn convert(&self, values: ValueRange) -> Result<Self::Output, sheets_connector::error::Error> {
        let mut assets_count = 0;
        
        if let Some(values) = values.values {
            for data in values {
                println!("Values: {:#?}", &data);
            }
        }

        Ok(vec![])
    }
}