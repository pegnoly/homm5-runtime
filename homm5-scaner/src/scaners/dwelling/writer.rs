use std::{collections::HashMap, fs::File, io::Write};
use zip::ZipWriter;

use crate::{
    core::Output,
    error::ScanerError,
    scaners::dwelling::scaner::{DwellingLobbyData, DwellingScanerOutput},
};

pub struct DwellingDataOutput {
    entities: Vec<DwellingScanerOutput>,
}

impl DwellingDataOutput {
    pub fn new() -> Self {
        DwellingDataOutput { entities: vec![] }
    }
}

impl Default for DwellingDataOutput {
    fn default() -> Self {
        Self::new()
    }
}

impl Output for DwellingDataOutput {
    type Input = DwellingScanerOutput;

    fn output_single(&mut self, object: Self::Input) -> Result<(), ScanerError> {
        self.entities.push(object);
        Ok(())
    }

    async fn finish_output(&self, _zip_writer: &mut ZipWriter<File>) -> Result<(), ScanerError> {
        let lobby_data: HashMap<super::model::DwellingType, DwellingLobbyData> = HashMap::from_iter(
            self.entities
                .iter()
                .map(|dwell| (dwell.dwell_type.clone(), dwell.data.clone())),
        );
        let mut json_file = std::fs::File::create("D:\\dwellings.json")?;
        let json_string = serde_json::to_string_pretty(&lobby_data)?;
        json_file.write_all(json_string.as_bytes())?;

        Ok(())
    }
}
