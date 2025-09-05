use sheets_connector::service::SheetsConnectorService;
use tauri::State;

use crate::error::Error;

#[tauri::command]
pub async fn upload_to_sheets(
    sheets_connector: State<'_, SheetsConnectorService>
) -> Result<(), Error> {
    let data = sheets_connector.convert_xlsx("D:\\2.xlsx").await?;
    sheets_connector.upload_to_sheets(data).await?;
    Ok(())
}