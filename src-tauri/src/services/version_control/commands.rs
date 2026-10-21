use tauri::State;
use tokio::sync::Mutex;
use editor_tools::prelude::FightGeneratorRepo;
use homm5_scaner::prelude::ScanerService;
use crate::error::Error;
use crate::services::version_control::{manage_pulled_data, VersionControlService};
use crate::utils::LocalAppManager;

#[tauri::command]
pub async fn pull_data_from_sheets(
    app_manager: State<'_, LocalAppManager>,
    version_control: State<'_, Mutex<VersionControlService>>,
    scaner_service: State<'_, ScanerService>,
    fight_generator_repo: State<'_, FightGeneratorRepo>
) -> Result<(), Error> {
    let mut version_control_locked = version_control.lock().await;
    let pull_data = version_control_locked.pull(&app_manager).await?;
    manage_pulled_data(
        &mut version_control_locked, 
        &app_manager,
        &scaner_service,
        &fight_generator_repo,
        &pull_data
    ).await?;
    Ok(())
}