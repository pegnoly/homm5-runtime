use std::str::FromStr;
use homm5_scaner::prelude::{ArtifactClassType, ArtifactDBModel, ArtifactSlotType, FromSheetArtifactUpdateModel};
use itertools::Itertools;
use serde_json::Value;
use google_api::prelude::ValueRange;
use google_api::service::GoogleApiService;
use google_api::utils::{FromSheetValueRange, IntoSheetsData};
use google_api::error::Error;

pub struct ArtifactsSheetsConverter;

impl IntoSheetsData<Vec<ValueRange>> for ArtifactsSheetsConverter {
    type Input = Vec<ArtifactDBModel>;

    fn convert_into_sheets_data(&self, source: Self::Input) -> Result<Vec<ValueRange>, Error> {
        let mut result = vec![];

        for model in source {
            result.push(ValueRange {
                major_dimension: Some("ROWS".to_string()),
                range: Some(format!("artifacts_data!A{}:L{}", model.id + 1, model.id + 1)),
                values: Some(vec![vec![
                    Value::Number(model.id.into()),
                    Value::String(model.game_id),
                    Value::String(model.name),
                    Value::String(model.slot.to_string()),
                    Value::String(model.class.to_string()),
                    Value::Number(model.cost.into()),
                    Value::Bool(model.is_generatable),
                    Value::String(format!("{}, {}, {}, {}, {}, {}", model.attack, model.defence, model.spellpower, model.knowledge, model.morale, model.luck)),
                    Value::String(model.name_txt),
                    Value::String(model.desc_txt),
                    Value::String(model.icon_xdb),
                    Value::String(model.desc)
                ]])
            })
        }

        Ok(result)
    }
}

impl FromSheetValueRange<Vec<ValueRange>> for ArtifactsSheetsConverter {
    type Output = Vec<FromSheetArtifactUpdateModel>;

    fn convert_from_sheets_data(&self, values: Vec<ValueRange>) -> Result<Self::Output, google_api::error::Error> {
        let mut result = vec![];
        for value in values {
            let data = value.values.and_then(|row| row.first().cloned())
                .ok_or_else(|| Error::UndefinedValue("Artifact data can't be read from sheet".to_string()))?;
            let stats_data = data.get(7)
                .and_then(|v| match v {
                    Value::String(s) => Some(s.split(',').map(|s| s.trim().parse::<i32>().unwrap_or(0)).collect_vec()),
                    _=> None
                })
                .ok_or_else(|| Error::UndefinedValue("Artifact cost can't be read from sheet".to_string()))?;
            let model = FromSheetArtifactUpdateModel {
                id: data.first()
                    .and_then(|v| match v {
                        Value::String(s) => Some(s.parse().unwrap()),
                        _=> None
                    })
                    .ok_or_else(|| Error::UndefinedValue("Artifact id can't be read from sheet".to_string()))?,
                game_id: data.get(1)
                    .and_then(|v| match v {
                        Value::String(s) => Some(s.clone()),
                        _=> None
                    })
                    .ok_or_else(|| Error::UndefinedValue("Artifact game id can't be read from sheet".to_string()))?,
                name: data.get(2)
                    .and_then(|v| match v {
                        Value::String(s) => Some(s.clone()),
                        _=> None
                    })
                    .ok_or_else(|| Error::UndefinedValue("Artifact name can't be read from sheet".to_string()))?,
                slot: data.get(3)
                    .and_then(|v| match v {
                        Value::String(s) => Some(ArtifactSlotType::from_str(&s).unwrap()),
                        _=> None
                    })
                    .ok_or_else(||Error::UndefinedValue("Artifact slot can't be read from sheet".to_string()))?,
                class: data.get(4)
                    .and_then(|v| match v {
                        Value::String(s) => Some(ArtifactClassType::from_str(&s).unwrap()),
                        _=> None
                    })
                    .ok_or_else(||Error::UndefinedValue("Artifact class can't be read from sheet".to_string()))?,
                cost: data.get(5)
                    .and_then(|v| match v {
                        Value::String(s) => Some(s.parse().unwrap()),
                        _=> None
                    })
                    .ok_or_else(||Error::UndefinedValue("Artifact cost can't be read from sheet".to_string()))?,
                is_generatable: data.get(6)
                    .and_then(|v| match v {
                        Value::String(s) => Some(s.to_lowercase().parse().unwrap()),
                        _=> None
                    })
                    .ok_or_else(||Error::UndefinedValue("Artifact is_generatable param can't be read from sheet".to_string()))?,
                attack: stats_data[0],
                defence: stats_data[1],
                spellpower: stats_data[2],
                knowledge: stats_data[3],
                morale: stats_data[4],
                luck: stats_data[5],
                name_txt: data.get(8)
                    .and_then(|v| match v {
                        Value::String(s) => Some(s.clone()),
                        _=> None
                    })
                    .ok_or_else(||Error::UndefinedValue("Artifact name path can't be read from sheet".to_string()))?,
                desc_txt: data.get(9)
                    .and_then(|v| match v {
                        Value::String(s) => Some(s.clone()),
                        _=> None
                    })
                    .ok_or_else(||Error::UndefinedValue("Artifact desc path can't be read from sheet".to_string()))?,
                icon_xdb: data.get(10)
                    .and_then(|v| match v {
                        Value::String(s) => Some(s.clone()),
                        _=> None
                    })
                    .ok_or_else(||Error::UndefinedValue("Artifact icon path can't be read from sheet".to_string()))?,
                desc: data.get(11)
                    .and_then(|v| match v {
                        Value::String(s) => Some(s.clone()),
                        _=> None
                    })
                    .ok_or_else(||Error::UndefinedValue("Artifact desc can't be read from sheet".to_string()))?,
            };
            result.push(model);
        }

        Ok(result)
    }
}

pub async fn sync_artifacts(sheets_connector: &GoogleApiService, models: Vec<ArtifactDBModel>) -> Result<(), Error> {
    sheets_connector
        .upload_to_sheet_bulk("1rCt_qVXPhzFHZwgTwk8DnLyCfIOP1JzW7gbZ22AGhwQ", models, ArtifactsSheetsConverter{})
        .await?;
    Ok(())
}

pub async fn pull_artifacts(sheets_connector: &GoogleApiService) -> Result<(), Error> {
    let data = sheets_connector
        .read_many_from_sheet(
            "1rCt_qVXPhzFHZwgTwk8DnLyCfIOP1JzW7gbZ22AGhwQ",
            vec!["artifacts_data!A3:L3"],
            ArtifactsSheetsConverter{}
        )
        .await?;
    println!("Artifacts pulled {:?}", data);
    Ok(())
}