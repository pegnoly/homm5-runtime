use std::sync::LazyLock;
use itertools::Itertools;
use sheets_connector::service::SheetsConnectorService;
use tauri::State;
use homm5_scaner::prelude::{ArtifactSlotType, GetArtifactsPayload, ScanerService};
use sheets_connector::prelude::ValueRange;
use crate::error::Error;
use crate::utils::LocalAppManager;

static COUNT_CALC_RULES: LazyLock<Vec<&str>> = LazyLock::new(|| {
   vec!["Power based [0]", "Raw [1]"]
});

static GROW_RULES: LazyLock<Vec<&str>> = LazyLock::new(|| {
   vec!["Static [0]", "Dynamic [1]"]
});

static GENERATION_RULES: LazyLock<Vec<&str>> = LazyLock::new(|| {
   vec!["Tier-slot based [0]", "Specific unit [1]"]
});

static TOWNS: LazyLock<Vec<&str>> = LazyLock::new(|| {
   vec![
       "Haven [0]",
       "Inferno [1]",
       "Necropolis [2]",
       "Preserve [3]",
       "Dungeon [4]",
       "Academy [5]",
       "Fortress [6]",
       "Stronghold [7]",
       "Bastion [8]",
       "Sanctuary [9]",
       "Renegades [10]",
       "Neutral [-1]"
   ]
});

static TYPE_BASED_RULES: LazyLock<Vec<&str>> = LazyLock::new(|| {
   vec!["Only generatable [0]", "Only upgrade [1]", "Only shooter [2]", "Only caster [3]"]
});

static STAT_BASED_RULES: LazyLock<Vec<&str>> = LazyLock::new(|| {
   vec![
       "MinBy hp [0]",
       "MaxBy hp [1]",
       "MinBy initiative [2]",
       "MaxBy initiative [3]",
       "MinBy speed [4]",
       "MaxBy speed [5]",
       "MinBy attack [6]",
       "MaxBy attack [7]",
       "MinBy defence [8]",
       "MaxBy defence [9]"
   ]
});

static ARTIFACT_SLOTS_QUEUE: LazyLock<Vec<ArtifactSlotType>> = LazyLock::new(|| {
    vec![
        ArtifactSlotType::Chest,
        ArtifactSlotType::Feet,
        ArtifactSlotType::Finger,
        ArtifactSlotType::Head,
        ArtifactSlotType::Miscslot1,
        ArtifactSlotType::Neck,
        ArtifactSlotType::Primary,
        ArtifactSlotType::Secondary,
        ArtifactSlotType::Secondary,
    ]
});

#[tauri::command]
pub async fn upload_to_sheets(
    sheets_connector: State<'_, SheetsConnectorService>,
) -> Result<(), Error> {
    let _data = sheets_connector
        .convert_xlsx("D:\\2.xlsx")
        .await
        .map_err(|e| Error::SheetsConnector(Box::new(e)))?;
    // sheets_connector.upload_to_sheets(data).await?;
    Ok(())
}

#[tauri::command]
pub async fn generate_validation_data(
    sheets_connector: State<'_, SheetsConnectorService>,
    scaner_service: State<'_, ScanerService>,
    app_manager: State<'_, LocalAppManager>,
) -> Result<(), Error> {
    let generatable_creatures = scaner_service.get_creature_models().await?;
    let artifacts = scaner_service.get_artifact_models(GetArtifactsPayload::default().with_generatable(true)).await?;
    let mut data: Vec<Vec<serde_json::Value>> = vec![
        COUNT_CALC_RULES.iter().map(|x| serde_json::Value::String(x.to_string())).collect_vec(),
        GROW_RULES.iter().map(|x| serde_json::Value::String(x.to_string())).collect_vec(),
        GENERATION_RULES.iter().map(|x| serde_json::Value::String(x.to_string())).collect_vec(),
        TOWNS.iter().map(|x| serde_json::Value::String(x.to_string())).collect_vec(),
        (1..8).map(|x| serde_json::Value::String(format!("Tier {x} [{}]", x - 1))).collect_vec(),
        TYPE_BASED_RULES.iter().map(|x| serde_json::Value::String(x.to_string())).collect_vec(),
        STAT_BASED_RULES.iter().map(|x| serde_json::Value::String(x.to_string())).collect_vec(),
        //
        generatable_creatures.iter().map(|x| {
            serde_json::Value::String(format!("{} [{}]", if let Some(inner_name) = &x.inner_name {
                if !inner_name.is_empty() { inner_name } else { &x.name }
            } else {
                &x.name
            }, x.id))
        }).collect_vec(),
        //
        artifacts.iter().map(|x| {
            serde_json::Value::String(format!("{} [{}]", x.name, x.id))
        }).collect_vec(),
    ];

    for slot in ARTIFACT_SLOTS_QUEUE.iter() {
        data.push(artifacts.iter().filter(|a| a.slot == *slot).map(|a| {
            serde_json::Value::String(format!("{} [{}]", a.name, a.id))
        }).collect_vec());
    }

    let output = ValueRange {
        major_dimension: Some("COLUMNS".to_string()),
        range: Some(format!("Validation!A3:R{}", generatable_creatures.len() + 2)),
        values: Some(data),
    };

    let profile = app_manager.current_profile_data.read().await;
    let current_map = app_manager
        .runtime_config
        .read()
        .await
        .current_selected_map
        .unwrap();
    let spreadsheet_id = &profile
        .maps
        .iter()
        .find(|map| map.id == current_map)
        .ok_or(Error::UndefinedData(String::from("Current map")))?
        .fights_spreadsheet_id;
    
    sheets_connector.upload_validation_data(spreadsheet_id, output).await.map_err(Box::new)?;
    
    Ok(())
}