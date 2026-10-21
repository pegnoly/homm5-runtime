use std::{fmt::Debug, path::{Path, PathBuf}};
use crate::{error::Error, service::types::SheetCreationResponse};
use calamine::{Reader, Xlsx, open_workbook};
use google_docs1::api::{AddDocumentTabRequest, BatchUpdateDocumentRequest, CreateNamedRangeRequest, DeleteContentRangeRequest, InsertTextRequest, Location, Range, Request, TabProperties};
use google_docs1::{Docs, yup_oauth2};
use google_docs1::hyper_rustls::HttpsConnectorBuilder;
use google_docs1::hyper_util::client::legacy::Client;
use google_docs1::hyper_util::rt::TokioExecutor;
use google_sheets4::{
    Sheets,
    api::{DimensionProperties, RowData, ValueRange}
};
use google_sheets4::api::{BatchUpdateValuesRequest, Spreadsheet};
use google_sheets4::hyper_rustls::HttpsConnector;
use google_sheets4::hyper_util::client::legacy::connect::HttpConnector;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_json::Value;

mod types;

pub struct GoogleApiService {
    sheets_hub: tokio::sync::Mutex<Sheets<HttpsConnector<HttpConnector>>>,
    docs_hub: tokio::sync::Mutex<Docs<HttpsConnector<HttpConnector>>>,
    reqwest_client: reqwest::Client,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RowTransferData {
    pub index: i32,
    pub row: Vec<RowData>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ColumnTransferData {
    pub index: i32,
    pub dimensions: DimensionProperties,
}

pub type SheetId = i32;

const APP_SCRIPT_URL: &str = "https://script.google.com/macros/s/AKfycbzZn1Un3oVkQKpkuasn_Zk8P16OzX3IN8BzjcDgH6-eb2cJREvJ21ePI7RHfws4WF2qmA/exec";

impl GoogleApiService {
    pub async fn new(client_secret_path: &PathBuf) -> Result<Self, Error> {
        let secret = yup_oauth2::read_application_secret(client_secret_path).await?;
        let connector = HttpsConnectorBuilder::new()
            .with_native_roots()?  // or .with_webpki_roots()
            .https_or_http()        // Allows both HTTP and HTTPS
            .enable_http1()
            .build();
        let executor = TokioExecutor::new();
        let client = Client::builder(executor.clone())
            .build(connector.clone());
        let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
                secret,
                yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
                yup_oauth2::client::CustomHyperClientBuilder::from(
                    Client::builder(executor).build(connector),
                )
            )
            .persist_tokens_to_disk(
                client_secret_path.parent().unwrap().join("token.json")
            )
            .build()
            .await?;

        Ok(GoogleApiService {
            sheets_hub: tokio::sync::Mutex::new(Sheets::new(client.clone(), auth.clone())),
            docs_hub: tokio::sync::Mutex::new(Docs::new(client, auth)),
            reqwest_client: reqwest::Client::new(),
        })
    }

    async fn single_update(&self, spreadsheet_id: &str, values: ValueRange) -> Result<(), Error> {
        let hub_locked = self.sheets_hub.lock().await;
        hub_locked
            .spreadsheets()
            .values_update(values.clone(), spreadsheet_id, &values.range.unwrap())
            .value_input_option("USER_ENTERED")
            .doit()
            .await
            .map_err(|e| Error::Api(Box::new(e)))?;
        Ok(())
    }

    async fn bulk_update(&self, spreadsheet_id: &str, values: Vec<ValueRange>) -> Result<(), Error> {
        let hub_locked = self.sheets_hub.lock().await;
        hub_locked
            .spreadsheets()
            .values_batch_update(BatchUpdateValuesRequest {
                data: Some(values),
                value_input_option: Some("USER_ENTERED".to_string()),
                ..Default::default()
            }, spreadsheet_id)
            .doit()
            .await
            .map_err(|e| Error::Api(Box::new(e)))?;
        Ok(())
    }

    async fn get_sheet_name(&self, spreadsheet: &Spreadsheet, sheet_id: i32) -> Result<String, Error> {
        let sheet_name = spreadsheet
            .sheets
            .as_ref()
            .and_then(|sheets| {
                sheets.iter().find(|sheet| {
                    sheet
                        .properties
                        .as_ref()
                        .and_then(|props| props.sheet_id)
                        .is_some_and(|id| id == sheet_id)
                })
            })
            .and_then(|sheet| sheet.properties.as_ref())
            .and_then(|properties| properties.title.as_ref())
            .ok_or(Error::UndefinedValue(
                "read_from_sheet Sheet name".to_string(),
            ))?;

        Ok(sheet_name.clone())
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

    pub async fn create_sheet(
        &self,
        spreadsheet_id: &str,
        sheet_name: &str,
    ) -> Result<SheetId, Error> {
        let response = self
            .reqwest_client
            .post(format!("{APP_SCRIPT_URL}?action=createSheet"))
            .json(&serde_json::json!({"spreadsheetId": spreadsheet_id, "sheetName": sheet_name}))
            .send()
            .await?
            .json::<SheetCreationResponse>()
            .await?;

        Ok(response.created_sheet_id)
    }

    pub async fn update_sheet<T: Serialize + Debug>(
        &self,
        action: &str,
        post_data: &T
    ) -> Result<(), Error> {
        println!("Updating sheet with action {action} and data {post_data:#?}");
        let response = self.reqwest_client
            .post(format!("{APP_SCRIPT_URL}?action={action}"))
            .json(post_data)
            .send()
            .await?;

        println!("Update response: {:#?}", response.text().await?);

        Ok(())
    }

    pub async fn upload_to_sheet(
        &self,
        spreadsheet_id: &str,
        values: ValueRange
    ) -> Result<(), Error> {
        self.single_update(spreadsheet_id, values).await?;
        Ok(())
    }

    pub async fn upload_to_sheet_bulk(
        &self,
        spreadsheet_id: &str,
        values: Vec<ValueRange>,
    ) -> Result<(), Error> {
        self.bulk_update(spreadsheet_id, values).await?;
        Ok(())
    }

    pub async fn append_to_sheet(
        &self,
        spreadsheet_id: &str,
        sheet_name: &str,
        value: ValueRange,
    ) -> Result<(), Error> {
        let hub_locked = self.sheets_hub.lock().await;
        hub_locked
            .spreadsheets()
            .values_append(value.clone(), spreadsheet_id, &format!("{}!A1", sheet_name))
            .value_input_option("USER_ENTERED")
            .doit()
            .await.map_err(|e| Error::Api(Box::new(e)))?;
        Ok(())
    }

    pub async fn read_from_sheet(
        &self,
        spreadsheet_id: &str,
        sheet_id: i32,
        range: &str
    ) -> Result<ValueRange, Error> {
        let hub_locked = self.sheets_hub.lock().await;
        let response = hub_locked
            .spreadsheets()
            .get(spreadsheet_id)
            .doit()
            .await
            .map_err(|e| Error::Api(Box::new(e)))?;

        let sheet_name = self.get_sheet_name(&response.1, sheet_id).await?;

        let data = hub_locked
            .spreadsheets()
            .values_get(spreadsheet_id, &format!("{sheet_name}!{range}"))
            .major_dimension("COLUMNS")
            .doit()
            .await
            .map_err(|e| Error::Api(Box::new(e)))?;

        Ok(data.1)
    }

    pub async fn read_many_from_sheet(
        &self,
        spreadsheet_id: &str,
        ranges: Vec<String>,
        dimension: &str
    ) -> Result<Option<Vec<ValueRange>>, Error> {
        let hub_locked = self.sheets_hub.lock().await;

        let mut request = hub_locked
            .spreadsheets()
            .values_batch_get(spreadsheet_id);

        for range in &ranges {
            request = request.add_ranges(range);
        }

        let data = request
            .major_dimension(dimension)
            .doit()
            .await
            .map_err(|e| Error::Api(Box::new(e)))?;

        Ok(data.1.value_ranges)
    }

    pub async fn read_all_rows(
        &self,
        spreadsheet_id: &str,
        sheet_name: &str,
        column: &str
    ) -> Result<Vec<Vec<Value>>, Error> {
        let hub_locked = self.sheets_hub.lock().await;
        let response = hub_locked
            .spreadsheets()
            .values_get(spreadsheet_id, &format!("{sheet_name}!A:{column}"))
            .doit()
            .await
            .map_err(|e| Error::Api(Box::new(e)))?;

        Ok(if let Some(data) = response.1.values {
            data.into_iter().filter(|d| !d.is_empty()).collect_vec()
        } else {
            vec![]
        })
    }

    pub async fn upload_validation_data(&self, spreadsheet_id: &str, data: ValueRange) -> Result<(), Error> {
        self.single_update(spreadsheet_id, data).await?;
        Ok(())
    }

    pub async fn create_document_tab(&self, document_id: &str, tab_name: &str) -> Result<String, Error> {
        let hub_locked = self.docs_hub.lock().await;
        let result = hub_locked.documents()
            .batch_update(BatchUpdateDocumentRequest {
                requests: Some(vec![
                    Request {
                        add_document_tab: Some(AddDocumentTabRequest {
                            tab_properties: Some(TabProperties {
                                title: Some(tab_name.to_string()),
                                ..Default::default()
                            })
                        }),
                        ..Default::default()
                    }
                ]),
                ..Default::default()
            }, document_id)
            .doit()
            .await
            .map_err(|e| Error::Api(Box::new(e)))?;

        let replies = result.1.replies.map(|replies| replies.into_iter().filter(|r| r.add_document_tab.is_some()).collect::<Vec<_>>())
            .unwrap_or_else(Vec::new);

        let id = replies.iter().find(|r| r.add_document_tab.as_ref().unwrap().tab_properties.as_ref().unwrap().title.as_ref().unwrap() == tab_name)
            .as_ref().unwrap()
            .add_document_tab
            .as_ref().unwrap()
            .tab_properties
            .as_ref().unwrap()
            .tab_id
            .as_ref()
            .unwrap();

        Ok(id.clone())
    }

    pub async fn fill_tab_with_text(&self, document_id: &str, tab_id: &str) -> Result<(), Error> {
        let hub_locked = self.docs_hub.lock().await;

        let data = hub_locked.documents()
            .get(document_id)
            .include_tabs_content(true)
            .doit()
            .await
            .map_err(|e| Error::Api(Box::new(e)))?;

        let tab = data.1.tabs
            .and_then(|tabs| tabs.iter().find(|tab| tab.tab_properties.as_ref().unwrap().tab_id.as_ref().unwrap() == tab_id).cloned())
            .ok_or(Error::UndefinedValue("No tab found".to_string()))?;

        let body_content = tab
            .document_tab.as_ref()
            .and_then(|dt| dt.body.as_ref())
            .and_then(|b| b.content.as_ref())
            .ok_or_else(|| Error::UndefinedValue("No body content".to_string()))?;

        let segment_end = body_content
            .iter()
            .filter_map(|e| e.end_index)
            .max()
            .ok_or_else(|| Error::UndefinedValue("Empty segment".to_string()))?;

        let mut ranges = body_content
            .iter()
            .filter_map(|e| e.paragraph.as_ref())
            .filter_map(|p| p.elements.as_ref())
            .flatten()
            .filter_map(|el| {
                if el.text_run.is_some() {
                    Some((el.start_index?, el.end_index?))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for (_, end) in ranges.iter_mut() {
            if *end >= segment_end {
                *end -= 1;
            }
        }

        ranges.retain(|(start, end)| start < end);
        ranges.sort_by(|a, b| b.0.cmp(&a.0));

        let delete_requests= ranges
            .into_iter()
            .map(|(start, end)| Request {
                delete_content_range: Some(DeleteContentRangeRequest {
                    range: Some(Range {
                        tab_id: Some(tab_id.to_string()),
                        start_index: Some(start),
                        end_index: Some(end),   // <-- без +1
                        ..Default::default()
                    }),
                }),
                ..Default::default()
            })
            .collect();

        hub_locked.documents()
            .batch_update(BatchUpdateDocumentRequest {
                requests: Some(delete_requests),
                ..Default::default()
            }, document_id)
            .doit()
            .await
            .map_err(|e| Error::Api(Box::new(e)))?;

        hub_locked.documents()
            .batch_update(BatchUpdateDocumentRequest {
                requests: Some(vec![
                    Request {
                        insert_text: Some(InsertTextRequest {
                            text: Some("Тестовый текст".to_string()),
                            location: Some(Location {
                                tab_id: Some(tab_id.to_string()),
                                index: Some(1),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }
                ]),
                ..Default::default()
            }, document_id)
            .doit()
            .await
            .map_err(|e| Error::Api(Box::new(e)))?;

        Ok(())
    }

    pub async fn add_named_ranges_to_tab(&self, document_id: &str, tab_id: &str, ranges: Vec<&str>) -> Result<(), Error> {
        let hub_locked = self.docs_hub.lock().await;

        let data = hub_locked.documents()
            .get(document_id)
            .include_tabs_content(true)
            .doit()
            .await
            .map_err(|e| Error::Api(Box::new(e)))?;

        let tab = data.1.tabs
            .and_then(|tabs| tabs.iter().find(|tab| tab.tab_properties.as_ref().unwrap().tab_id.as_ref().unwrap() == tab_id).cloned())
            .ok_or(Error::UndefinedValue("No tab found".to_string()))?;

        let body_content = tab
            .document_tab.as_ref()
            .and_then(|dt| dt.body.as_ref())
            .and_then(|b| b.content.as_ref())
            .ok_or_else(|| Error::UndefinedValue("No body content".to_string()))?;

        let ranges = body_content
            .iter()
            .filter_map(|e| e.paragraph.as_ref())
            .filter_map(|p| p.elements.as_ref())
            .flatten()
            .filter_map(|el| {
                if let Some(item) = &el.text_run {
                    let text = item.content.as_ref().unwrap();
                    if let Some(range) = ranges.iter().find(|r| text.starts_with(**r)) {
                        Some((el.start_index?, el.start_index? + range.len() as i32))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        println!("Ranges: {:?}", ranges);

        let requests = ranges
            .iter()
            .map(|r| Request {
                create_named_range: Some(CreateNamedRangeRequest {
                name: Some(format!("Range{}{}", r.0, r.1)),
                range: Some(Range {
                    tab_id: Some(tab_id.to_string()),
                    start_index: Some(r.0),
                    end_index: Some(r.1),
                    ..Default::default()
                })
            }),
            ..Default::default() })
            .collect::<Vec<_>>();

        hub_locked.documents()
            .batch_update(BatchUpdateDocumentRequest {
                requests: Some(requests),
                ..Default::default()
            }, document_id)
            .doit()
            .await
            .map_err(|e| Error::Api(Box::new(e)))?;

        Ok(())
    }
}
