use std::{io::Write, path::PathBuf};
use std::collections::HashMap;
use editor_tools::prelude::{
    CreateDialogPayload, CreateDialogVariantPayload, CreateSpeakerPayload, DialogGeneratorRepo,
    DialogModel, DialogVariantModel, GetDialogVariantPayload, SaveVariantPayload, SpeakerModel,
    SpeakerType, UpdateLabelsPayload,
};
use itertools::Itertools;
use tauri::{AppHandle, Emitter, State};
use tauri_plugin_dialog::DialogExt;

use crate::{error::Error, utils::LocalAppManager};
use crate::services::dialog_generator::types::DialogStepModel;

#[tauri::command]
pub async fn load_dialogs(
    app_manager: State<'_, LocalAppManager>,
    dialog_generator_repo: State<'_, DialogGeneratorRepo>,
) -> Result<Vec<DialogModel>, Error> {
    let current_map_id = app_manager
        .runtime_config
        .read()
        .await
        .current_selected_map;
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
    let profile = app_manager.current_profile_data.read().await;
    let current_map_id = app_manager
        .runtime_config
        .read()
        .await
        .current_selected_map;
    let map = profile
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
        .current_selected_map;
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
    speakers: Vec<i32>,
    text: String,
) -> Result<(), Error> {
    Ok(dialog_generator_repo
        .save_variant(SaveVariantPayload { id, text, speakers })
        .await?)
}

#[tauri::command]
pub async fn generate_dialog(
    app_manager: State<'_, LocalAppManager>,
    dialog_generator_repo: State<'_, DialogGeneratorRepo>,
    dialog_id: i32,
) -> Result<(), Error> {
    let profile = app_manager.current_profile_data.read().await;

    if let Some(dialog) = dialog_generator_repo.get_dialog(dialog_id).await? {
        let speakers = dialog_generator_repo
            .get_speakers_by_ids(dialog.speakers_ids.ids)
            .await?;
        let variants = dialog_generator_repo
            .get_all_variants_for_dialog(dialog_id)
            .await?;
        let dialog_local_path = dialog.directory.replace(profile.map_path.to_str().unwrap(), "");
        let dialog_texts_path = profile.texts_path.join(&dialog_local_path);

        std::fs::create_dir_all(&dialog_texts_path)?;

        let mut script_file =
            std::fs::File::create(format!("{}\\script.lua", dialog.directory))?;

        let steps_count = variants.iter().unique_by(|v| v.step).count();
        let mut script = format!("{} = MiniDialog({{\n", dialog.script_name);
        script += &format!("\tpath = \"{}\",\n", &dialog_local_path.replace("\\", "/"));
        script += &format!("\tsteps_count = {},\n\tcurrent_step = 0,\n", steps_count);

        let mut steps: HashMap<i32, DialogStepModel> = HashMap::new();

        for variant in &variants {
            let file_name = format!("{}_{}.txt", &variant.step, &variant.label);
            let mut variant_file = std::fs::File::create(dialog_texts_path.join(file_name))?;
            let mut text = format!("<color=<value=speaker_color>><value=speaker_name><color=white>: {}", &variant.text);
            text = text.replace("<b>", "<font face=Header size=20>").replace("</b>", "<font face=Default size=20>");
            variant_file.write_all(&[255, 254])?;
            for utf16 in text.encode_utf16() {
                variant_file
                    .write_all(&(bincode::serialize(&utf16).unwrap()))?;
            }
            if let Some(step_data) = steps.get_mut(&variant.step) {
                step_data.labels.push(&variant.label);
            } else {
                steps.insert(variant.step, DialogStepModel {
                    speakers: variant.speaker_ids.ids.iter().map(|s| speakers.iter().find(|sp| sp.id == *s).unwrap().script_name.as_str()).collect(),
                    labels: vec![&variant.label],
                });
            }
        }

        script += &format!("\tsteps = {{{}}},\n", steps.values().map(|v| v.to_string()).join(", "));
        script += &format!("\tspeakers_data = {{{}}}\n",
                           speakers.iter().map(|s| {
                               format!("[{}] = {{color = \"{}\", type = {}}}", if s.speaker_type == SpeakerType::Hero {
                                   format!("\"{}\"", &s.script_name)
                               } else {
                                   s.script_name.clone()
                               }, s.color, s.speaker_type)
                           }).collect_vec().join(", "));

        script += "})\n\n__end_import()";
        script_file.write_all(script.as_bytes())?;

        if !dialog.was_generated {
            dialog_generator_repo.set_dialog_was_generated(dialog_id).await?;
        }
    }

    Ok(())
}
