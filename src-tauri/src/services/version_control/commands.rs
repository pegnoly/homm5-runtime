use tauri::State;
use tokio::sync::Mutex;
use crate::error::Error;
use crate::services::fight_generator::utils::AssetArmySlotSheetWrapper;
use crate::services::version_control::VersionControlService;
use crate::utils::LocalAppManager;

#[tauri::command]
pub async fn pull_data_from_sheets(
    app_manager: State<'_, LocalAppManager>,
    version_control: State<'_, Mutex<VersionControlService>>,
) -> Result<(), Error> {
    let version_control_locked = version_control.lock().await;
    let pull_data = version_control_locked.pull(&app_manager).await?;

    let chunks = pull_data.fights.chunks(3).collect::<Vec<_>>();

    for chunk in &chunks {
        for stack_data in &chunk[0] {
            let stack = AssetArmySlotSheetWrapper::try_from(stack_data.clone())?;
            println!("Stack pulled: {:#?}", stack);
        }
    }

    Ok(())
}