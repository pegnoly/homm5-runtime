use std::{fs::OpenOptions, io::Write, path::PathBuf};

use tauri::{AppHandle, Emitter, State};
use tauri_plugin_dialog::DialogExt;
use uuid::Uuid;

use crate::{
    DialogGeneratorService,
    utils::{Config, LocalAppManager},
};

use super::data::{DialogFrontendModel, SpeakerFrontendModel, SpeakerType};

#[tauri::command]
pub async fn load_dialogs(
    dialog_generator_service: State<'_, DialogGeneratorService>,
) -> Result<Vec<DialogFrontendModel>, ()> {
    match dialog_generator_service.get_dialogs().await {
        Ok(dialogs) => Ok(dialogs),
        Err(_error) => Err(()),
    }
}

#[tauri::command]
pub async fn load_speakers(
    dialog_generator_service: State<'_, DialogGeneratorService>,
) -> Result<Vec<SpeakerFrontendModel>, ()> {
    match dialog_generator_service.get_speakers().await {
        Ok(speakers) => Ok(speakers),
        Err(_error) => Err(()),
    }
}

#[tauri::command]
pub async fn pick_dialog_directory(
    app: AppHandle,
    config: State<'_, Config>,
    app_manager: State<'_, LocalAppManager>,
) -> Result<(), ()> {
    let current_map_id = app_manager
        .runtime_config
        .read()
        .await
        .current_selected_map
        .unwrap();
    let map = config.maps.iter().find(|m| m.id == current_map_id).unwrap();

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
    dialog_generator_service: State<'_, DialogGeneratorService>,
    name: String,
    script_name: String,
    directory: String,
    speakers: Vec<String>,
) -> Result<DialogFrontendModel, ()> {
    match dialog_generator_service
        .create_dialog(&name, &script_name, &directory, &speakers)
        .await
    {
        Ok(dialog) => Ok(dialog),
        Err(_error) => Err(()),
    }
}

#[tauri::command]
pub async fn create_speaker(
    dialog_generator_service: State<'_, DialogGeneratorService>,
    name: String,
    script_name: String,
    color: String,
    speaker_type: SpeakerType,
) -> Result<SpeakerFrontendModel, ()> {
    match dialog_generator_service
        .create_speaker(&name, &script_name, &color, speaker_type)
        .await
    {
        Ok(speaker) => Ok(speaker),
        Err(_error) => Err(()),
    }
}

#[tauri::command]
pub async fn update_dialog_labels(
    dialog_generator_service: State<'_, DialogGeneratorService>,
    dialog_id: Uuid,
    labels: Vec<String>,
) -> Result<(), ()> {
    match dialog_generator_service
        .update_dialog_labels(dialog_id, &labels)
        .await
    {
        Ok(()) => Ok(()),
        Err(error) => {
            println!(
                "Failed to update dialog {} with labels {:?}: {}",
                dialog_id,
                &labels,
                error.to_string()
            );
            Err(())
        }
    }
}

#[tauri::command]
pub async fn load_dialog_name(
    dialog_generator_service: State<'_, DialogGeneratorService>,
    dialog_id: Uuid,
) -> Result<String, ()> {
    match dialog_generator_service.get_dialog_name(dialog_id).await {
        Ok(name) => Ok(name),
        Err(_error) => Err(()),
    }
}

#[tauri::command]
pub async fn load_dialog_script_name(
    dialog_generator_service: State<'_, DialogGeneratorService>,
    dialog_id: Uuid,
) -> Result<String, ()> {
    match dialog_generator_service
        .get_dialog_script_name(dialog_id)
        .await
    {
        Ok(script_name) => Ok(script_name),
        Err(_error) => Err(()),
    }
}

#[tauri::command]
pub async fn load_dialog_directory(
    dialog_generator_service: State<'_, DialogGeneratorService>,
    dialog_id: Uuid,
) -> Result<String, ()> {
    match dialog_generator_service
        .get_dialog_directory(dialog_id)
        .await
    {
        Ok(directory) => Ok(directory),
        Err(_error) => Err(()),
    }
}

#[tauri::command]
pub async fn load_dialog_speakers(
    dialog_generator_service: State<'_, DialogGeneratorService>,
    dialog_id: Uuid,
) -> Result<Vec<Uuid>, ()> {
    match dialog_generator_service
        .get_dialog_speakers(dialog_id)
        .await
    {
        Ok(speakers) => Ok(speakers),
        Err(_error) => Err(()),
    }
}

#[tauri::command]
pub async fn load_dialog_labels(
    dialog_generator_service: State<'_, DialogGeneratorService>,
    dialog_id: Uuid,
) -> Result<Vec<String>, ()> {
    match dialog_generator_service.get_dialog_labels(dialog_id).await {
        Ok(labels) => Ok(labels),
        Err(_error) => Err(()),
    }
}

#[tauri::command]
pub async fn load_dialog_variant(
    dialog_generator_service: State<'_, DialogGeneratorService>,
    dialog_id: Uuid,
    dialog_step: u32,
    label: String,
) -> Result<Uuid, ()> {
    match dialog_generator_service
        .get_dialog_variant_id(dialog_id, dialog_step, &label)
        .await
    {
        Ok(id) => Ok(id),
        Err(error) => {
            println!(
                "Error occured while loading dialog variant: {}",
                error.to_string()
            );
            Err(())
        }
    }
}

#[tauri::command]
pub async fn load_variant_text(
    dialog_generator_service: State<'_, DialogGeneratorService>,
    variant_id: Uuid,
) -> Result<String, ()> {
    match dialog_generator_service
        .get_dialog_variant_text(variant_id)
        .await
    {
        Ok(text) => Ok(text),
        Err(_error) => Err(()),
    }
}

#[tauri::command]
pub async fn load_variant_speaker(
    dialog_generator_service: State<'_, DialogGeneratorService>,
    variant_id: Uuid,
) -> Result<Option<Uuid>, ()> {
    match dialog_generator_service
        .get_dialog_variant_speaker_id(variant_id)
        .await
    {
        Ok(speaker_id) => match speaker_id {
            Some(speaker_id) => Ok(Some(speaker_id)),
            None => Ok(None),
        },
        Err(error) => {
            println!(
                "Failed to load speaker id for variant {}: {}",
                variant_id,
                error.to_string()
            );
            Err(())
        }
    }
}

#[tauri::command]
pub async fn save_dialog_variant(
    dialog_generator_service: State<'_, DialogGeneratorService>,
    variant_id: Uuid,
    speaker: Uuid,
    text: String,
) -> Result<(), ()> {
    match dialog_generator_service
        .save_dialog_variant(variant_id, speaker, &text)
        .await
    {
        Ok(_) => Ok(()),
        Err(error) => {
            println!(
                "Failed to save variant {}: {}",
                variant_id,
                error.to_string()
            );
            Err(())
        }
    }
}

#[tauri::command]
pub async fn generate_dialog(
    dialog_generator_service: State<'_, DialogGeneratorService>,
    app_manager: State<'_, LocalAppManager>,
    dialog_id: Uuid,
) -> Result<(), ()> {
    let current_map = app_manager
        .runtime_config
        .read()
        .await
        .current_selected_map
        .unwrap();

    let base_config_data = app_manager.base_config.read().await;
    let map_data = base_config_data.maps.iter().find(|m| m.id == current_map).unwrap();
    let map_data_path = &map_data.data_path;

    // get dialog data
    let dialog = dialog_generator_service
        .get_dialog(dialog_id)
        .await
        .unwrap();
    // get speakers
    let speakers = dialog_generator_service
        .get_speakers_by_ids(&serde_json::from_str(&dialog.speakers_ids).unwrap())
        .await
        .unwrap();
    // get all variants
    let variants = dialog_generator_service
        .get_variants_for_dialog(dialog_id)
        .await
        .unwrap();

    let dialog_local_path = dialog.directory.replace(&base_config_data.mod_path, "");
    let dialog_texts_path = format!("{}\\{}", &base_config_data.texts_path, &dialog_local_path);

    std::fs::create_dir_all(&dialog_texts_path).unwrap();

    let mut script_file =
        std::fs::File::create(format!("{}\\script.lua", dialog.directory)).unwrap();
    let mut script = format!("MiniDialog.Sets[\"{}\"] = {{\n", dialog.script_name);

    for variant in &variants {
        let file_name = format!("{}_{}.txt", &variant.step, &variant.label);
        let mut variant_file =
            std::fs::File::create(format!("{}\\{}", dialog_texts_path, file_name)).unwrap();
        if let Some(speaker) = speakers.iter().find(|s| s.id == variant.speaker_id) {
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
                format!("{}", speaker.script_name)
            };
            script += &format!(
                "\t[\"{}_{}\"] = {{speaker = {}, speaker_type = {}}},\n",
                &variant.step,
                &variant.label,
                speaker_script,
                speaker.speaker_type.to_string()
            );
        }
    }

    script += "}\n\n";
    script_file.write_all(&mut script.as_bytes()).unwrap();

    if !dialog.was_generated {
        dialog_generator_service
            .set_dialog_was_generated(dialog.id, true)
            .await
            .unwrap();

        let path_script = &format!(
            "MiniDialog.Paths[\"{}\"] = \"{}\"\n",
            dialog.script_name,
            &dialog_local_path.replace("\\", "/")
        );
        let mut paths_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(format!("{}dialogs_paths.lua", map_data_path))
            .unwrap();

        paths_file.write_all(path_script.as_bytes()).unwrap();
    }

    Ok(())
}
