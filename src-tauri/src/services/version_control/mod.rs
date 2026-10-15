use std::collections::HashMap;
use chrono::{DateTime, Local, Utc};
use dashmap::DashSet;
use serde::{Deserialize, Serialize};
use strum::Display;
use tauri::State;
use google_api::error::Error::UndefinedValue;
use google_api::service::GoogleApiService;
use google_api::utils::{FromSheetsData, IntoSheetsData};
use homm5_scaner::prelude::ScanerService;
use crate::error::Error;
use crate::services::artifact_editor::utils::push_artifacts_data;
use crate::utils::{LocalAppManager, TimelineMessage};

#[derive(Debug, Serialize, Deserialize, Display, Eq, PartialEq, Hash)]
pub enum VersionControlContent {
    Creatures,
    Artifacts,
    Speakers,
    Fights
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionControlCommit {
    pub created_at: DateTime<Utc>,
    pub author: String,
    pub pulled_by: Vec<String>,
    pub content: HashMap<VersionControlContent, Vec<i32>>,
}

pub struct VersionControlService {
    google_api: &'static GoogleApiService,
    pub updated_creatures: DashSet<i32>,
    pub updated_artifacts: DashSet<i32>
}

impl VersionControlService {
    pub fn new(google_api_service: &'static GoogleApiService) -> Self {
        Self {
            google_api: google_api_service,
            updated_creatures: DashSet::new(),
            updated_artifacts: DashSet::new()
        }
    }

     pub async fn push_single<D, C: IntoSheetsData<D>>(
        &self,
        spreadsheet_id: &str,
        data: D,
        converter: C
    ) -> Result<(), Error> {
        let value= converter.into_single(data)
            .map_err(|e| Error::GoogleApi(Box::new(e)))?;
        self.google_api.upload_to_sheet(spreadsheet_id, value).await
            .map_err(|e| Error::GoogleApi(Box::new(e)))?;
        Ok(())
    }

    pub async fn push_many<D, C: IntoSheetsData<D>>(
        &self,
        spreadsheet_id: &str,
        data: Vec<D>,
        converter: C
    ) -> Result<(), Error> {
        let values= converter.into_many(data)
            .map_err(|e| Error::GoogleApi(Box::new(e)))?;
        self.google_api.upload_to_sheet_bulk(spreadsheet_id, values).await
            .map_err(|e| Error::GoogleApi(Box::new(e)))?;
        Ok(())
    }

    pub async fn pull_single<D, C: FromSheetsData<D>>(
        &self,
        spreadsheet_id: &str,
        sheet_id: i32,
        range: &str,
        converter: C
    ) -> Result<D, Error> {
        let data = self.google_api.read_from_sheet(spreadsheet_id, sheet_id, range).await
            .map_err(|e| Error::GoogleApi(Box::new(e)))?;
        Ok(converter.from_single(data).map_err(|e| Error::GoogleApi(Box::new(e)))?)
    }

    pub async fn pull_many<D, C: FromSheetsData<D>>(
        &self,
        spreadsheet_id: &str,
        sheet_id: i32,
        ranges: Vec<&str>,
        converter: C
    ) -> Result<Vec<D>, Error> {
        let data = self.google_api.read_many_from_sheet(spreadsheet_id, sheet_id, ranges).await
            .map_err(|e| Error::GoogleApi(Box::new(e)))?;

        if let Some(values) = data {
            Ok(converter.from_many(values).map_err(|e| Error::GoogleApi(Box::new(e)))?)
        } else {
            Err(Error::GoogleApi(Box::new(UndefinedValue("No data was read from sheet".to_string()))))
        }
    }
}

#[tauri::command]
pub async fn send_data_to_sheets(
    app_manager: State<'_, LocalAppManager>,
    version_control: State<'_, VersionControlService>,
    scaner_service: State<'_, ScanerService>
) -> Result<TimelineMessage, Error> {
    let base_config = app_manager.base_config.read().await;
    let mut commit = VersionControlCommit {
        created_at: Utc::now(),
        author: base_config.author.clone(),
        pulled_by: vec![],
        content: HashMap::new()
    };

    if !version_control.updated_artifacts.is_empty() {
        let updated_ids = push_artifacts_data(&app_manager, &version_control, &scaner_service).await?;
        commit.content.insert(VersionControlContent::Artifacts, updated_ids);
    }

    if !version_control.updated_creatures.is_empty() {

    }

    Ok(TimelineMessage {
        timestamp: Local::now().format("%d:%m:%Y - %H:%M").to_string(),
        message: format!("Commit created: {:#?}", &commit)
    })
}