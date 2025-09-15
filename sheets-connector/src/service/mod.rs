use std::{path::{Path, PathBuf}, time::Duration};

use calamine::{open_workbook, Reader, Xlsx};
use google_sheets4::{api::{BatchUpdateValuesRequest, DimensionProperties, RowData, Sheet, SheetProperties, Spreadsheet, SpreadsheetProperties, ValueRange}, hyper_rustls::{self, HttpsConnector}, hyper_util::{self, client::legacy::connect::HttpConnector}, yup_oauth2, FieldMask, Sheets};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::time::timeout;

use crate::{error::Error, service::types::SheetCreationResponse, utils::*};

mod types;

pub struct SheetsConnectorService {
    sheets_hub: tokio::sync::Mutex<Sheets<HttpsConnector<HttpConnector>>>,
    reqwest_client: reqwest::Client
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RowTransferData {
    pub index: i32,
    pub row: Vec<RowData>
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ColumnTransferData {
    pub index: i32,
    pub dimensions: DimensionProperties
}

pub type SheetId = i32;

const APP_SCRIPT_URL: &str = "https://script.google.com/macros/s/AKfycbzZn1Un3oVkQKpkuasn_Zk8P16OzX3IN8BzjcDgH6-eb2cJREvJ21ePI7RHfws4WF2qmA/exec";

impl SheetsConnectorService {
    pub async fn new(client_secret_path: &PathBuf) -> Result<Self, Error> {
        let secret = yup_oauth2::read_application_secret(client_secret_path).await?;
        let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
            secret,
            yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
        )
            .persist_tokens_to_disk("D:\\projects\\homm5-runtime\\editor-tools\\target\\tokens\\token.json")
            .build()
            .await?;

        let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
            .build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .unwrap()
                    .https_or_http()
                    .enable_http1()
                    .build(),
            );
            
        Ok(SheetsConnectorService {
            sheets_hub: tokio::sync::Mutex::new(Sheets::new(client, auth)),
            reqwest_client: reqwest::Client::new()
        })
    }

    pub async fn convert_xlsx<T: AsRef<Path>>(&self, path: T) -> Result<Vec<Vec<String>>, Error> {
        let mut xlsx: Xlsx<_> = open_workbook(path)?;
        let mut data = vec![];
        if let Ok(r) = xlsx.worksheet_range("Общая статистика по расам") {
            for row in r.rows() {
                let string_row: Vec<String> = row.iter().map(|s| s.to_string()).collect();
                data.push(string_row);
            }
        } 
        Ok(data)
    }

    pub async fn create_sheet(&self, spreadsheet_id: &str, sheet_name: &str) -> Result<SheetId, Error> {
        let response = self.reqwest_client.post(format!("{APP_SCRIPT_URL}?action=createSheet"))
            .json(&serde_json::json!({"spreadsheetId": spreadsheet_id, "sheetName": sheet_name}))
            .send()
            .await?
            .json::<SheetCreationResponse>()
            .await?;

        Ok(response.created_sheet_id)
    }

    pub async fn upload_to_sheet<C, I>(&self, spreadsheet_id: &str, input: I, converter: C) -> Result<(), Error>
        where C: IntoSheetsData<ValueRange, Input = I>
    {
        let values = converter.into_sheets_data(input)?;
        let hub_locked = self.sheets_hub.lock().await;
        hub_locked.spreadsheets()
            .values_update(values.clone(), spreadsheet_id, &values.range.unwrap())
            .value_input_option("USER_ENTERED")
            .doit()
            .await?;
        Ok(())
    }

    pub async fn read_from_sheet<T, V>(&self, spreadsheet_id: &str, sheet_id: i32, range: &str, converter: T) -> Result<V, Error>
        where T: FromSheetValueRange<Output = V>
    {
        let hub_locked = self.sheets_hub.lock().await;
        let spreadsheet = hub_locked.spreadsheets()
            .get(spreadsheet_id)
            .doit()
            .await?;
        let sheet_name = spreadsheet.1.sheets
            .as_ref()
            .and_then(|sheets| {
                    sheets.iter().find(|sheet| {
                        sheet.properties
                            .as_ref()
                            .and_then(|props| props.sheet_id)
                            .is_some_and(|id| id == sheet_id)
                    })
                }
            )
            .and_then(|sheet| sheet.properties.as_ref())
            .and_then(|properties| properties.title.as_ref())
            .ok_or(Error::UndefinedValue("read_from_sheet Sheet name".to_string()))?;

        let data = hub_locked.spreadsheets()
            .values_get(spreadsheet_id, &format!("{sheet_name}!{range}"))
            .major_dimension("COLUMNS")
            .doit()
            .await?;

        let value = converter.from_value_range(data.1)?;
        
        Ok(value)

    }
}