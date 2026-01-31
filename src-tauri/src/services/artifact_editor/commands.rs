use std::{fs::File, io::Write, path::PathBuf};
use homm5_scaner::prelude::{ArtifactClassType, ArtifactSlotType, GetArtifactsPayload, ScanerService, UpdateArtifactPayload};
use homm5_types::art::{AdvMapArtifactShared, ArtifactObject, Table_DBArtifact_ArtifactEffect};
use itertools::Itertools;
use quick_xml::{Writer, events::{BytesDecl, Event}};
use tauri::{AppHandle, Emitter, State};
use tauri_plugin_dialog::DialogExt;
use zip::write::FileOptions;
use crate::{error::Error, utils::LocalAppManager};

#[tauri::command]
pub async fn update_artefact_attack(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: i32
) -> Result<(), Error> {
    Ok(scaner_service.update_artefact(UpdateArtifactPayload::new(id).with_attack(value)).await?)
}

#[tauri::command]
pub async fn update_artefact_defence(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: i32
) -> Result<(), Error> {
    Ok(scaner_service.update_artefact(UpdateArtifactPayload::new(id).with_defence(value)).await?)
}

#[tauri::command]
pub async fn update_artefact_spell_power(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: i32
) -> Result<(), Error> {
    Ok(scaner_service.update_artefact(UpdateArtifactPayload::new(id).with_spell_power(value)).await?)
}

#[tauri::command]
pub async fn update_artefact_knowledge(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: i32
) -> Result<(), Error> {
    Ok(scaner_service.update_artefact(UpdateArtifactPayload::new(id).with_knowledge(value)).await?)
}

#[tauri::command]
pub async fn update_artefact_luck(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: i32
) -> Result<(), Error> {
    Ok(scaner_service.update_artefact(UpdateArtifactPayload::new(id).with_luck(value)).await?)
}

#[tauri::command]
pub async fn update_artefact_morale(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: i32
) -> Result<(), Error> {
    Ok(scaner_service.update_artefact(UpdateArtifactPayload::new(id).with_morale(value)).await?)
}

#[tauri::command]
pub async fn update_artefact_cost(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: i32
) -> Result<(), Error> {
    Ok(scaner_service.update_artefact(UpdateArtifactPayload::new(id).with_cost(value)).await?)
}

#[tauri::command]
pub async fn update_artefact_class(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: ArtifactClassType
) -> Result<(), Error> {
    Ok(scaner_service.update_artefact(UpdateArtifactPayload::new(id).with_class(value)).await?)
}

#[tauri::command]
pub async fn update_artefact_slot(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: ArtifactSlotType
) -> Result<(), Error> {
    Ok(scaner_service.update_artefact(UpdateArtifactPayload::new(id).with_slot(value)).await?)
}

#[tauri::command]
pub async fn select_artefact_name_path(
    app: AppHandle,
    app_manager: State<'_, LocalAppManager>,
) -> Result<(), Error> {
    let profile = app_manager.current_profile_data.read().await;
    let path = profile.texts_path.clone();
    app.dialog()
        .file()
        .set_directory(PathBuf::from(&format!("{}Text\\Game\\Artifacts\\NAF\\", path)))
        .set_can_create_directories(true)
        .pick_folder(move |f| {
            app.emit("artifact_name_path_selected", f.unwrap().to_string().replace(&path, "")).unwrap();
        });
    Ok(())
}

#[tauri::command]
pub async fn update_artefact_name_path(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: String
) -> Result<(), Error> {
    Ok(scaner_service.update_artefact(UpdateArtifactPayload::new(id).with_name_txt(value)).await?)
}

#[tauri::command]
pub async fn select_artefact_desc_path(
    app: AppHandle,
    app_manager: State<'_, LocalAppManager>,
) -> Result<(), Error> {
    let profile = app_manager.current_profile_data.read().await;
    let path = profile.texts_path.clone();
    app.dialog()
        .file()
        .set_directory(PathBuf::from(&format!("{}Text\\Game\\Artifacts\\NAF\\", path)))
        .set_can_create_directories(true)
        .pick_folder(move |f| {
            app.emit("artifact_desc_path_selected", f.unwrap().to_string().replace(&path, "")).unwrap();
        });
    Ok(())
}

#[tauri::command]
pub async fn update_artefact_desc_path(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: String
) -> Result<(), Error> {
    Ok(scaner_service.update_artefact(UpdateArtifactPayload::new(id).with_desc_txt(value)).await?)
}

#[tauri::command]
pub async fn select_artefact_icon_path(
    app: AppHandle,
    app_manager: State<'_, LocalAppManager>,
) -> Result<(), Error> {
    let profile = app_manager.current_profile_data.read().await;
    let path = profile.data_path.clone();
    app.dialog()
        .file()
        .set_directory(PathBuf::from(&format!("{}GOG_Mod\\Textures\\Icons\\Artifacts\\", &path)))
        .set_can_create_directories(true)
        .pick_folder(move |f| {
            app.emit("artifact_icon_path_selected", f.unwrap().to_string().replace(&format!("{}GOG_Mod\\", &path), "")).unwrap();
        });
    Ok(())
}

#[tauri::command]
pub async fn update_artefact_icon_path(
    scaner_service: State<'_, ScanerService>,
    app_manager: State<'_, LocalAppManager>,
    id: i32,
    value: String,
    path: String
) -> Result<(), Error> {
    let profile = app_manager.current_profile_data.read().await;
    let base_cfg = app_manager.base_config.read().await;
    let icon_xdb_path = PathBuf::from(format!("{}GOG_Mod\\{}", profile.data_path, path));
    if !icon_xdb_path.exists() {
        let icon_xdb = base_cfg.generic_icon_128.as_ref().unwrap();
        let icon_dds = base_cfg.generic_icon_dds.as_ref().unwrap();
        std::fs::copy(icon_xdb, &icon_xdb_path)?;
        std::fs::copy(icon_dds, icon_xdb_path.to_str().unwrap().replace(".xdb", ".dds"))?;
    }
    Ok(scaner_service.update_artefact(UpdateArtifactPayload::new(id).with_icon_xdb(value)).await?)
}


#[tauri::command]
pub async fn update_artefact_name(
    scaner_service: State<'_, ScanerService>,
    app_manager: State<'_, LocalAppManager>,
    id: i32,
    value: String,
    path: String
) -> Result<(), Error> {
    let profile = app_manager.current_profile_data.read().await;
    let path = PathBuf::from(format!("{}{}", profile.texts_path, path));
    let mut file = File::create(path)?;
    file.write_all(&[255, 254])?;
    for utf16 in value.encode_utf16() {
        file.write_all(&(bincode::serialize(&utf16).unwrap())).unwrap();
    }
    Ok(scaner_service.update_artefact(UpdateArtifactPayload::new(id).with_name(value)).await?)
}

#[tauri::command]
pub async fn update_artefact_desc(
    scaner_service: State<'_, ScanerService>,
    app_manager: State<'_, LocalAppManager>,
    id: i32,
    value: String,
    path: String
) -> Result<(), Error> {
    let profile = app_manager.current_profile_data.read().await;
    let path = PathBuf::from(format!("{}{}", profile.texts_path, path));
    let mut file = File::create(path)?;
    file.write_all(&[255, 254])?;
    for utf16 in value.encode_utf16() {
        file.write_all(&(bincode::serialize(&utf16).unwrap())).unwrap();
    }
    Ok(scaner_service.update_artefact(UpdateArtifactPayload::new(id).with_desc(value)).await?)
}

#[tauri::command]
pub async fn update_artefact_texts_paths(
    scaner_service: State<'_, ScanerService>,
    id: i32,
    value: String
) -> Result<(), Error> {
    let name_txt = format!("{}/name.txt", &value.to_lowercase().replace("\\", "/"));
    let desc_txt = format!("{}/name.txt", &value.to_lowercase().replace("\\", "/"));
    scaner_service.update_artefact(UpdateArtifactPayload::new(id).with_name_txt(name_txt)).await?;
    scaner_service.update_artefact(UpdateArtifactPayload::new(id).with_desc_txt(desc_txt)).await?;
    Ok(())
}

#[tauri::command]
pub async fn rebuild_artifacts_file(
    scaner_service: State<'_, ScanerService>,
    app_manager: State<'_, LocalAppManager>
) -> Result<(), Error> {
    let profile_locked = app_manager.current_profile_data.read().await;
    let artifacts = scaner_service.get_artifact_models(GetArtifactsPayload::default()).await?;
    let table = Table_DBArtifact_ArtifactEffect {
        objects: artifacts.into_iter().map(|art| {
            ArtifactObject {
                ID: art.game_id.clone(),
                obj: AdvMapArtifactShared::from(art)
            }
        }).collect_vec()
    };
    let universe_pak_path = PathBuf::from(format!("{}Universe_mod.pak", profile_locked.data_path));
    let temp_pak_path = PathBuf::from(format!("{}Universe_mod_temp.pak", profile_locked.data_path));
    let temp_file = File::create(&temp_pak_path)?;
    let old_file = File::open(&universe_pak_path)?;
    let mut old_archive = zip::ZipArchive::new(old_file).unwrap();
    let mut new_archive = zip::ZipWriter::new(temp_file);

    for i in 0..old_archive.len() {
        let entry = old_archive.by_index(i).unwrap();
        if entry.name() != "GameMechanics/RefTables/Artifacts.xdb" {
            new_archive.raw_copy_file(entry).unwrap();
        }
    }

    let mut output: Vec<u8> = Vec::new();
    let mut writer = Writer::new_with_indent(&mut output, b' ', 4);
    writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;
    writer.create_element("Table_DBArtifact_ArtifactEffect").write_inner_content(|w| {
        w.write_serializable("objects", &table).unwrap();
        Ok(())
    })?;
    new_archive.start_file("GameMechanics/RefTables/Artifacts.xdb", FileOptions::default()).unwrap();
    new_archive.write_all(&output)?;
    new_archive.finish().unwrap();
    std::fs::rename(temp_pak_path, universe_pak_path)?;
    Ok(())
}