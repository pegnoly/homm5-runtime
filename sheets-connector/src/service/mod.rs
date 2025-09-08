use std::path::{Path, PathBuf};

use calamine::{open_workbook, Reader, Xlsx};
use google_sheets4::{api::{AddSheetRequest, BatchUpdateSpreadsheetRequest, GridCoordinate, Request, RowData, Sheet, SheetProperties, Spreadsheet, SpreadsheetProperties, UpdateCellsRequest, ValueRange}, hyper_rustls::{self, HttpsConnector}, hyper_util::{self, client::legacy::connect::HttpConnector}, yup_oauth2, FieldMask, Sheets};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{error::Error, utils::{SheetsValueRangeConverter}};

pub struct SheetsConnectorService {
    sheets_hub: tokio::sync::Mutex<Sheets<HttpsConnector<HttpConnector>>>
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RowTransferData {
    pub index: i32,
    pub row: Vec<RowData>
}

pub type SheetId = i32;

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

    pub async fn create_sheet(&self, spreadsheet_id: &str, sheet_name: &str) -> Result<SheetId, Error> {
        let hub_locked = self.sheets_hub.lock().await;

        let example_sheet_data = hub_locked.spreadsheets()
            .get(spreadsheet_id)
            .add_ranges("Example!A1:H24")
            .param("fields", "sheets(data(rowData(values(userEnteredFormat,effectiveFormat,dataValidation,userEnteredValue,hyperlink,note,textFormatRuns))))")
            .doit()
            .await?.1;

        let mut rows_count = 0;
        let mut rows_data = vec![];
        if let Some(sheets) = example_sheet_data.sheets {
            if let Some(sheet) = sheets.first() {
                if let Some(data) = &sheet.data {
                    for grid_data in data {
                        if let Some(row_data) = &grid_data.row_data {
                            rows_data.push(RowTransferData {
                                index: rows_count,
                                row: row_data.clone()
                            });
                            rows_count+=1;
                        }
                    }
                }
            }
        }

        let sheet_creation_response = hub_locked.spreadsheets()
            .batch_update(BatchUpdateSpreadsheetRequest {
                requests: Some(vec![Request {
                    add_sheet: Some(AddSheetRequest {
                        properties: Some(SheetProperties {
                            title: Some(sheet_name.to_string()),
                            ..Default::default()
                        }),
                    }),
                    ..Default::default()
                }]),
                ..Default::default()
            }, spreadsheet_id)
            .doit()
            .await?
            .1;

        let created_sheet_id = sheet_creation_response.replies
            .as_deref()
            .and_then(|replies| replies.first())
            .and_then(|reply| reply.add_sheet.as_ref())
            .and_then(|response| response.properties.as_ref())
            .and_then(|properties| properties.sheet_id)
            .unwrap();

        let mut requests = vec![];
        for row in rows_data {
            requests.push(Request {
                update_cells: Some(UpdateCellsRequest {
                    rows: Some(row.row),
                    start: Some(GridCoordinate {
                        column_index: Some(0),
                        row_index: Some(row.index),
                        sheet_id: Some(created_sheet_id)
                    }),
                    fields: Some(FieldMask::new(&[String::from("*")])),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }
        
        hub_locked.spreadsheets()
            .batch_update(BatchUpdateSpreadsheetRequest {
                requests: Some(requests),
                ..Default::default()
            }, spreadsheet_id)
            .param("fields", "*")
            .doit()
            .await?;

        Ok(created_sheet_id)
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

    pub async fn read_from_sheet<T, V>(&self, spreadsheet_id: &str, sheet_id: i32, range: &str, converter: T) -> Result<V, Error>
        where T: SheetsValueRangeConverter<Output = V>
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
            .doit()
            .await?;

        let value = converter.convert(data.1)?;
        Ok(value)

    }
}