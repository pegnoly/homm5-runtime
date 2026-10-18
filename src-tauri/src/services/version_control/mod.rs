use crate::error::Error;
use crate::services::artifact_editor::utils::commit_artifacts_data;
use crate::utils::{LocalAppManager, TimelineMessage};
use chrono::{DateTime, Local, Utc};
use dashmap::DashSet;
use google_api::error::Error::UndefinedValue;
use google_api::service::GoogleApiService;
use google_api::utils::{FromSheetsData, IntoSheetsData};
use homm5_scaner::prelude::ScanerService;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::Display;
use tauri::State;
use tokio::sync::Mutex;
use google_api::prelude::ValueRange;

#[derive(Debug, Serialize, Deserialize, Display, Eq, PartialEq, Hash)]
pub enum VersionControlContent {
    Creatures,
    Artifacts,
    Speakers,
    Fights,
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
    pub updated_artifacts: DashSet<i32>,
    pub updated_fights: DashSet<i32>,
    requests: HashMap<String, Vec<ValueRange>>,
}

impl VersionControlService {
    pub fn new(google_api_service: &'static GoogleApiService) -> Self {
        Self {
            google_api: google_api_service,
            updated_creatures: DashSet::new(),
            updated_artifacts: DashSet::new(),
            updated_fights: DashSet::new(),
            requests: HashMap::new(),
        }
    }

    pub async fn commit_single<D: Into<ValueRange>, C: IntoSheetsData<D>>(
        &mut self,
        spreadsheet_id: &str,
        data: D,
        converter: C,
    ) -> Result<(), Error> {
        let value = converter
            .into_single(data)
            .map_err(|e| Error::GoogleApi(Box::new(e)))?;
        if !self.requests.contains_key(spreadsheet_id) {
            self.requests.insert(spreadsheet_id.to_string(), vec![]);
        }
        self.requests.get_mut(spreadsheet_id).unwrap().push(value);
        Ok(())
    }

    pub async fn commit_many<D: Into<ValueRange>, C: IntoSheetsData<D>>(
        &mut self,
        spreadsheet_id: &str,
        data: Vec<D>,
        converter: C,
    ) -> Result<(), Error> {
        let values = converter
            .into_many(data)
            .map_err(|e| Error::GoogleApi(Box::new(e)))?;
        if !self.requests.contains_key(spreadsheet_id) {
            self.requests.insert(spreadsheet_id.to_string(), vec![]);
        }
        self.requests.get_mut(spreadsheet_id).unwrap().extend(values);
        Ok(())
    }

    pub async fn pull_single<D: TryFrom<ValueRange>, C: FromSheetsData<D>>(
        &self,
        spreadsheet_id: &str,
        sheet_id: i32,
        range: &str,
        converter: C,
    ) -> Result<D, Error> {
        let data = self
            .google_api
            .read_from_sheet(spreadsheet_id, sheet_id, range)
            .await
            .map_err(|e| Error::GoogleApi(Box::new(e)))?;
        Ok(converter
            .from_single(data)
            .map_err(|e| Error::GoogleApi(Box::new(e)))?)
    }

    pub async fn pull_many<D: TryFrom<ValueRange>, C: FromSheetsData<D>>(
        &self,
        spreadsheet_id: &str,
        sheet_id: i32,
        ranges: Vec<&str>,
        converter: C,
    ) -> Result<Vec<D>, Error> {
        let data = self
            .google_api
            .read_many_from_sheet(spreadsheet_id, sheet_id, ranges)
            .await
            .map_err(|e| Error::GoogleApi(Box::new(e)))?;

        if let Some(values) = data {
            Ok(converter
                .from_many(values)
                .map_err(|e| Error::GoogleApi(Box::new(e)))?)
        } else {
            Err(Error::GoogleApi(Box::new(UndefinedValue(
                "No data was read from sheet".to_string(),
            ))))
        }
    }

    pub async fn push(&mut self) -> Result<(), Error> {
        for (sheet, data) in &self.requests {
            self.google_api.upload_to_sheet_bulk(&sheet, data.clone()).await.map_err(|e| Error::GoogleApi(Box::new(e)))?;
        }
        Ok(())
    }
}

#[tauri::command]
pub async fn sync_data_with_sheets(
    app_manager: State<'_, LocalAppManager>,
    version_control: State<'_, Mutex<VersionControlService>>,
    scaner_service: State<'_, ScanerService>,
) -> Result<TimelineMessage, Error> {
    let base_config = app_manager.base_config.read().await;
    let mut commit = VersionControlCommit {
        created_at: Utc::now(),
        author: base_config.author.clone(),
        pulled_by: vec![],
        content: HashMap::new(),
    };

    let mut version_control_locked = version_control.lock().await;

    if !version_control_locked.updated_artifacts.is_empty() {
        let updated_ids =
            commit_artifacts_data(&app_manager, &mut version_control_locked, &scaner_service).await?;
        commit
            .content
            .insert(VersionControlContent::Artifacts, updated_ids);
    }

    if !version_control_locked.updated_creatures.is_empty() {}

    version_control_locked.push().await?;
    
    Ok(TimelineMessage {
        timestamp: Local::now().format("%d:%m:%Y - %H:%M").to_string(),
        message: format!("Commit created: {:#?}", &commit),
    })
}
