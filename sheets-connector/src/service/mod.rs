use std::path::{Path, PathBuf};

use calamine::{open_workbook, Reader, Xlsx};
use google_sheets4::{api::{Sheet, SheetProperties, Spreadsheet, SpreadsheetProperties, ValueRange}, hyper_rustls::{self, HttpsConnector}, hyper_util::{self, client::legacy::connect::HttpConnector}, yup_oauth2, Sheets};
use itertools::Itertools;
use serde_json::json;

use crate::error::Error;

pub struct SheetsConnectorService {
    pub sheets_hub: tokio::sync::Mutex<Sheets<HttpsConnector<HttpConnector>>>
}

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

        let hub = Sheets::new(client, auth);
        let data = hub.spreadsheets()
            .get("1dwAWSWrbMvxIrCoa3qvRrC6RqOWjNDmXtIFE7IhvBNU")
            .doit()
            .await
            .unwrap();

        println!("{:#?}", &data.1);

        Ok(SheetsConnectorService {
            sheets_hub: tokio::sync::Mutex::new(hub)
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

    pub async fn upload_to_sheets(&self, data: Vec<Vec<String>>) -> Result<(), Error> {
        let hub_locked = self.sheets_hub.lock().await;

        let values = data.into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|cell| json!(cell))
                    .collect_vec()
            })
            .collect_vec();

        let create_result = hub_locked.spreadsheets()
            .create(Spreadsheet {
                properties: Some(SpreadsheetProperties {
                    title: Some(String::from("Test spreadsheet")),
                    ..Default::default()
                }),
                sheets: Some(
                    vec![
                        Sheet {
                            properties: Some(SheetProperties {
                                title: Some(String::from("Test sheet")),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }
                    ]
                ),
                ..Default::default()
            })
            .doit()
            .await?;

        let update_result = hub_locked.spreadsheets()
            .values_update(
        ValueRange {
                    values: Some(values),
                    ..Default::default()
                }, 
                &create_result.1.spreadsheet_id.unwrap(), 
                create_result.1.sheets.unwrap().first().unwrap().properties.as_ref().unwrap().title.as_ref().unwrap()
            )
            .value_input_option("RAW")
            .doit()
            .await?;

        println!("Result: {:#?}", update_result.1);

        Ok(())
    }
}