use std::path::PathBuf;

use google_sheets4::{hyper_rustls::{self, HttpsConnector}, hyper_util::{self, client::legacy::connect::HttpConnector}, yup_oauth2, Sheets};

use crate::error::Error;

pub struct SheetsConnectorService {
    pub sheets_hub: Sheets<HttpsConnector<HttpConnector>>
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
            sheets_hub: hub
        })
    }
}