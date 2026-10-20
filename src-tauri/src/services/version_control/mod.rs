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
use std::sync::Arc;
use serde_json::Value;
use strum::Display;
use editor_tools::prelude::FightGeneratorRepo;
use google_api::prelude::ValueRange;
use crate::services::fight_generator::utils::commit_updated_fights;

#[derive(Debug, Serialize, Deserialize, Display, Eq, PartialEq, Hash)]
pub enum VersionControlSimpleType {
    Creatures,
    Artifacts,
    Speakers,
    Commits
}

#[derive(Debug, Serialize, Deserialize, Display)]
pub enum VersionControlContent {
    Creatures(Vec<i32>),
    Artifacts(Vec<i32>),
    Speakers(Vec<i32>),
    Fights(HashMap<String, Vec<String>>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionControlCommit {
    pub created_at: DateTime<Utc>,
    pub author: String,
    pub pulled_by: Vec<String>,
    pub content: Vec<VersionControlContent>,
}

#[allow(clippy::from_over_into)]
impl Into<ValueRange> for VersionControlCommit {
    fn into(self) -> ValueRange {
        ValueRange {
            major_dimension: Some("ROWS".to_string()),
            values: Some(vec![vec![
                Value::String(self.created_at.to_string()),
                Value::String(self.author),
                Value::String(String::new()),
                Value::String(serde_json::to_string(&self.content).unwrap())
            ]]),
            ..Default::default()
        }
    }
}

pub struct VersionControlService {
    google_api: Arc<GoogleApiService>,
    pub updated_creatures: DashSet<i32>,
    pub updated_artifacts: DashSet<i32>,
    pub updated_fights: DashSet<i32>,
    requests: HashMap<String, Vec<ValueRange>>,
}

impl VersionControlService {
    pub fn new(google_api_service: Arc<GoogleApiService>) -> Self {
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
        converter
            .from_single(data)
            .map_err(|e| Error::GoogleApi(Box::new(e)))
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

    pub async fn push(&mut self, commit: VersionControlCommit, app_manager: &LocalAppManager) -> Result<(), Error> {
        for (sheet, data) in &self.requests {
            self.google_api.upload_to_sheet_bulk(sheet, data.clone()).await.map_err(|e| Error::GoogleApi(Box::new(e)))?;
        }

        let profile = app_manager.current_profile_data.read().await;
        let sheet_data = profile.global_sheets.get(&VersionControlSimpleType::Commits).unwrap();
        self.google_api.append_to_sheet(&sheet_data.spreadsheet_id, "commits", commit.into()).await
            .map_err(|e| Error::GoogleApi(Box::new(e)))?;

        Ok(())
    }
}

pub fn get_sheet_name(value: &ValueRange) -> Result<String, Error> {
    let range = value.range.as_ref()
        .ok_or(Error::UndefinedData("ValueRange range".to_string()))?;
    Ok(range.split('!').collect::<Vec<&str>>()[0].to_string())
}

pub async fn sync_data_with_sheets(
    app_manager: &LocalAppManager,
    version_control: &mut VersionControlService,
    scaner_service: &ScanerService,
    fight_generator_repo: &FightGeneratorRepo
) -> Result<TimelineMessage, Error> {
    let base_config = app_manager.base_config.read().await;
    let mut commit = VersionControlCommit {
        created_at: Utc::now(),
        author: base_config.author.clone(),
        pulled_by: vec![],
        content: vec![],
    };

    if !version_control.updated_artifacts.is_empty() {
        let updated_ids =
            commit_artifacts_data(app_manager, version_control, scaner_service).await?;
        commit
            .content
            .push(VersionControlContent::Artifacts(updated_ids));
    }

    if !version_control.updated_creatures.is_empty() {}

    if !version_control.updated_fights.is_empty() {
        let updated_data = commit_updated_fights(version_control, scaner_service, app_manager, fight_generator_repo).await?;
        commit.content.push(VersionControlContent::Fights(updated_data));
    }

    let message = format!("Commit created: {:#?}", &commit);

    version_control.push(commit, app_manager).await?;
    
    Ok(TimelineMessage {
        timestamp: Local::now().format("%d:%m:%Y - %H:%M").to_string(),
        message
    })
}
