// use editor_tools::prelude::{SpeakerModel, SpeakerType};
// use google_api::error::Error;
// use google_api::prelude::ValueRange;
// use google_api::utils::{FromSheetValueRange, IntoSheetsData};
// use serde_json::Value;
// use std::str::FromStr;
// 
// pub struct SpeakersSheetConverter;
// 
// impl IntoSheetsData<Vec<ValueRange>> for SpeakersSheetConverter {
//     type Input = Vec<SpeakerModel>;
// 
//     fn convert_into_sheets_data(&self, source: Self::Input) -> Result<Vec<ValueRange>, Error> {
//         let mut result = vec![];
// 
//         for model in source {
//             result.push(ValueRange {
//                 major_dimension: Some("ROWS".to_string()),
//                 range: Some(format!("speakers_data!A{}:E{}", model.id + 1, model.id + 1)),
//                 values: Some(vec![vec![
//                     Value::Number(model.id.into()),
//                     Value::String(model.name),
//                     Value::String(model.script_name),
//                     Value::String(model.speaker_type.to_string()),
//                     Value::String(model.color),
//                     Value::Number(model.local_to_map.unwrap_or(-1).into()),
//                 ]]),
//             })
//         }
// 
//         Ok(result)
//     }
// }
// 
// impl FromSheetValueRange<Vec<ValueRange>> for SpeakersSheetConverter {
//     type Output = Vec<SpeakerModel>;
// 
//     fn convert_from_sheets_data(&self, values: Vec<ValueRange>) -> Result<Self::Output, Error> {
//         let mut result = vec![];
// 
//         for value in values {
//             let data = value
//                 .values
//                 .and_then(|row| row.first().cloned())
//                 .ok_or_else(|| {
//                     Error::UndefinedValue("Speakers data can't be read from sheet".to_string())
//                 })?;
// 
//             result.push(SpeakerModel {
//                 id: data
//                     .first()
//                     .and_then(|v| match v {
//                         Value::String(s) => Some(s.parse().unwrap()),
//                         _ => None,
//                     })
//                     .ok_or_else(|| {
//                         Error::UndefinedValue("Speaker id can't be read from sheet".to_string())
//                     })?,
//                 name: data
//                     .get(1)
//                     .and_then(|v| match v {
//                         Value::String(s) => Some(s.clone()),
//                         _ => None,
//                     })
//                     .ok_or_else(|| {
//                         Error::UndefinedValue("Speaker name can't be read from sheet".to_string())
//                     })?,
//                 script_name: data
//                     .get(2)
//                     .and_then(|v| match v {
//                         Value::String(s) => Some(s.clone()),
//                         _ => None,
//                     })
//                     .ok_or_else(|| {
//                         Error::UndefinedValue(
//                             "Speaker script name can't be read from sheet".to_string(),
//                         )
//                     })?,
//                 speaker_type: data
//                     .get(3)
//                     .and_then(|v| match v {
//                         Value::String(s) => Some(SpeakerType::from_str(&s).unwrap()),
//                         _ => None,
//                     })
//                     .ok_or_else(|| {
//                         Error::UndefinedValue("Speaker type can't be read from sheet".to_string())
//                     })?,
//                 color: data
//                     .get(4)
//                     .and_then(|v| match v {
//                         Value::String(s) => Some(s.clone()),
//                         _ => None,
//                     })
//                     .ok_or_else(|| {
//                         Error::UndefinedValue("Speaker color can't be read from sheet".to_string())
//                     })?,
//                 local_to_map: data
//                     .get(1)
//                     .and_then(|v| match v {
//                         Value::String(s) => {
//                             let n = s.parse::<i32>().unwrap();
//                             if n == -1 { None } else { Some(Some(n)) }
//                         }
//                         _ => None,
//                     })
//                     .ok_or_else(|| {
//                         Error::UndefinedValue("Speaker name can't be read from sheet".to_string())
//                     })?,
//             })
//         }
// 
//         Ok(result)
//     }
// }
