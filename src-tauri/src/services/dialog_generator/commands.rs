use std::{fs::OpenOptions, io::Write, path::PathBuf};

use editor_tools::prelude::{
    CreateDialogPayload, CreateDialogVariantPayload, CreateSpeakerPayload, DialogGeneratorRepo,
    DialogModel, DialogVariantModel, GetDialogVariantPayload, SaveVariantPayload, SpeakerModel,
    SpeakerType, UpdateLabelsPayload,
};
use itertools::Itertools;
use tauri::{AppHandle, Emitter, State};
use tauri_plugin_dialog::DialogExt;

use crate::{error::Error, utils::LocalAppManager};

#[tauri::command]
pub async fn load_dialogs(
    app_manager: State<'_, LocalAppManager>,
    dialog_generator_repo: State<'_, DialogGeneratorRepo>,
) -> Result<Vec<DialogModel>, Error> {
    let current_map_id = app_manager
        .runtime_config
        .read()
        .await
        .current_selected_map
        .unwrap();
    Ok(dialog_generator_repo
        .load_dialogs(current_map_id as i32)
        .await?)
}

#[tauri::command]
pub async fn load_speakers(
    dialog_generator_repo: State<'_, DialogGeneratorRepo>,
) -> Result<Vec<SpeakerModel>, Error> {
    let speakers = dialog_generator_repo.load_speakers().await?;
    Ok(speakers)
}

#[tauri::command]
pub async fn pick_dialog_directory(
    app: AppHandle,
    app_manager: State<'_, LocalAppManager>,
) -> Result<(), Error> {
    let base_config_locked = app_manager.base_config.read().await;
    let current_map_id = app_manager
        .runtime_config
        .read()
        .await
        .current_selected_map
        .unwrap();
    let map = base_config_locked
        .maps
        .iter()
        .find(|m| m.id == current_map_id)
        .unwrap();

    app.dialog()
        .file()
        .set_directory(PathBuf::from(&map.data_path))
        .set_can_create_directories(true)
        .pick_folder(move |f| {
            app.emit("dialog_directory_picked", f.unwrap().to_string())
                .unwrap();
        });
    Ok(())
}

#[tauri::command]
pub async fn create_new_dialog(
    app_manager: State<'_, LocalAppManager>,
    dialog_generator_repo: State<'_, DialogGeneratorRepo>,
    name: String,
    script_name: String,
    directory: String,
    speakers: Vec<i32>,
) -> Result<DialogModel, Error> {
    let current_map_id = app_manager
        .runtime_config
        .read()
        .await
        .current_selected_map
        .unwrap();
    Ok(dialog_generator_repo
        .create_dialog(CreateDialogPayload {
            mission_id: current_map_id as i32,
            name,
            script_name,
            directory,
            speakers,
        })
        .await?)
}

#[tauri::command]
pub async fn create_speaker(
    dialog_generator_repo: State<'_, DialogGeneratorRepo>,
    name: String,
    script_name: String,
    color: String,
    speaker_type: SpeakerType,
) -> Result<SpeakerModel, Error> {
    Ok(dialog_generator_repo
        .create_speaker(CreateSpeakerPayload {
            name,
            script_name,
            color,
            speaker_type,
        })
        .await?)
}

#[tauri::command]
pub async fn load_dialog(
    dialog_generator_repo: State<'_, DialogGeneratorRepo>,
    id: i32,
) -> Result<Option<DialogModel>, Error> {
    Ok(dialog_generator_repo.get_dialog(id).await?)
}

#[tauri::command]
pub async fn update_dialog_labels(
    dialog_generator_repo: State<'_, DialogGeneratorRepo>,
    dialog_id: i32,
    labels: Vec<String>,
) -> Result<(), Error> {
    Ok(dialog_generator_repo
        .update_dialog_labels(UpdateLabelsPayload { dialog_id, labels })
        .await?)
}

#[tauri::command]
pub async fn add_dialog_speaker(
    dialog_generator_repo: State<'_, DialogGeneratorRepo>,
    id: i32,
    speaker_id: i32,
) -> Result<(), Error> {
    Ok(dialog_generator_repo
        .update_dialog_speakers(id, speaker_id)
        .await?)
}

#[tauri::command]
pub async fn load_dialog_variant(
    dialog_generator_repo: State<'_, DialogGeneratorRepo>,
    dialog_id: i32,
    step: i32,
    label: String,
) -> Result<DialogVariantModel, Error> {
    if let Some(variant) = dialog_generator_repo
        .get_variant(GetDialogVariantPayload {
            dialog_id,
            step,
            label: label.clone(),
        })
        .await?
    {
        Ok(variant)
    } else {
        Ok(dialog_generator_repo
            .create_variant(CreateDialogVariantPayload {
                dialog_id,
                step,
                label,
            })
            .await?)
    }
}

#[tauri::command]
pub async fn save_dialog_variant(
    dialog_generator_repo: State<'_, DialogGeneratorRepo>,
    id: i32,
    speaker: i32,
    text: String,
) -> Result<(), Error> {
    Ok(dialog_generator_repo
        .save_variant(SaveVariantPayload { id, text, speaker })
        .await?)
}

#[tauri::command]
pub async fn generate_dialog(
    app_manager: State<'_, LocalAppManager>,
    dialog_generator_repo: State<'_, DialogGeneratorRepo>,
    dialog_id: i32,
) -> Result<(), Error> {
    let current_map = app_manager
        .runtime_config
        .read()
        .await
        .current_selected_map
        .unwrap();

    let base_config_data = app_manager.base_config.read().await;
    let map_data = base_config_data
        .maps
        .iter()
        .find(|m| m.id == current_map)
        .unwrap();
    let map_data_path = &map_data.data_path;

    if let Some(dialog) = dialog_generator_repo.get_dialog(dialog_id).await? {
        let speakers = dialog_generator_repo
            .get_speakers_by_ids(dialog.speakers_ids.ids)
            .await?;
        let variants = dialog_generator_repo
            .get_all_variants_for_dialog(dialog_id)
            .await?;
        let dialog_local_path = dialog.directory.replace(&base_config_data.mod_path, "");
        let dialog_texts_path = format!("{}\\{}", &base_config_data.texts_path, &dialog_local_path);

        std::fs::create_dir_all(&dialog_texts_path).unwrap();

        let mut script_file =
            std::fs::File::create(format!("{}\\script.lua", dialog.directory)).unwrap();
        let mut script = format!("MiniDialog.Sets[\"{}\"] = {{\n", dialog.script_name);

        for variant in &variants
            .iter()
            .filter(|v| v.speaker_id.is_some())
            .collect_vec()
        {
            let file_name = format!("{}_{}.txt", &variant.step, &variant.label);
            let mut variant_file =
                std::fs::File::create(format!("{dialog_texts_path}\\{file_name}")).unwrap();
            if let Some(speaker) = speakers
                .iter()
                .find(|s| s.id == variant.speaker_id.unwrap())
            {
                let text = format!(
                    "<color={}>{}<color=white>: {}",
                    &speaker.color, &speaker.name, &variant.text
                );
                variant_file.write_all(&[255, 254]).unwrap();
                for utf16 in text.encode_utf16() {
                    variant_file
                        .write_all(&(bincode::serialize(&utf16).unwrap()))
                        .unwrap();
                }
                let speaker_script = if speaker.speaker_type == SpeakerType::Hero {
                    format!("\"{}\"", speaker.script_name)
                } else {
                    speaker.script_name.to_string()
                };
                script += &format!(
                    "\t[\"{}_{}\"] = {{speaker = {}, speaker_type = {}}},\n",
                    &variant.step, &variant.label, speaker_script, speaker.speaker_type
                );
            }
        }

        script += "}\n\n";
        script_file.write_all(script.as_bytes()).unwrap();

        if !dialog.was_generated {
            dialog_generator_repo
                .set_dialog_was_generated(dialog_id)
                .await?;

            let path_script = &format!(
                "MiniDialog.Paths[\"{}\"] = \"{}\"\n",
                dialog.script_name,
                &dialog_local_path.replace("\\", "/")
            );
            let mut paths_file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(format!("{map_data_path}dialogs_paths.lua"))
                .unwrap();

            paths_file.write_all(path_script.as_bytes()).unwrap();
        }
    }

    Ok(())
}
