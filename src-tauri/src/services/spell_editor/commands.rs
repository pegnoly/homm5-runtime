use std::{io::{Read, Write}, path::PathBuf};

use homm5_scaner::prelude::{CreateSpellPayload, FileObject, FileObjects, MagicSchoolType, ScanerService, SpellDBModel};
use homm5_types::spell::SpellShared;
use itertools::Itertools;
use map_modifier::FileRef;
use quick_xml::{Reader, Writer, events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event}};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, State};
use tauri_plugin_dialog::DialogExt;
use zip::{read::ZipFile, write::FileOptions};

use crate::{error::Error, utils::LocalAppManager};

#[tauri::command]
pub async fn pick_spell_texts_directory(
    app: AppHandle,
    app_manager: State<'_, LocalAppManager>
) -> Result<(), Error> {
    let profile = app_manager.current_profile_data.read().await;
    let path = profile.texts_path.clone();
    app.dialog()
        .file()
        .set_directory(PathBuf::from(&format!("{}Text\\Game\\Spells\\", path)))
        .set_can_create_directories(true)
        .pick_folder(move |f| {
            if let Some(dir) = f {
                app.emit("spell_texts_directory_picked", dir.to_string().replace(&path, "")).ok();
            }
        });
    Ok(())
}

#[tauri::command]
pub async fn pick_spell_icon_directory(
    app: AppHandle,
    app_manager: State<'_, LocalAppManager>
) -> Result<(), Error> {
    let profile = app_manager.current_profile_data.read().await;
    let path = profile.data_path.clone();
    app.dialog()
        .file()
        .set_directory(PathBuf::from(&format!("{}GOG_Mod\\Textures\\Icons\\Spells\\", &path)))
        .set_can_create_directories(true)
        .pick_folder(move |f| {
            if let Some(dir) = f {
                app.emit("spell_icon_directory_picked", dir.to_string().replace(&format!("{}GOG_Mod\\", &path), "")).ok();
            }
        });
    Ok(())
}


#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub async fn create_new_spell(
    scaner_service: State<'_, ScanerService>,
    app_manager: State<'_, LocalAppManager>,
    game_id: String,
    name: String,
    desc: String,
    texts_path: String,
    icon_path: String,
    school: MagicSchoolType
) -> Result<SpellDBModel, Error> {
    let profile = app_manager.current_profile_data.read().await;
    let texts_dir = PathBuf::from(format!("{}{}\\", &profile.texts_path, &texts_path));
    let base_cfg = app_manager.base_config.read().await;
    if !texts_dir.exists()  {
        std::fs::create_dir_all(&texts_dir)?;
    }
    let name_path = texts_dir.join("name.txt");
    let desc_path = texts_dir.join("desc.txt");

    let mut name_file = std::fs::File::create(&name_path)?;
    name_file.write_all(&[255, 254])?;
    for utf16 in name.encode_utf16() {
        name_file.write_all(&(bincode::serialize(&utf16).unwrap())).unwrap();
    }

    let mut desc_file = std::fs::File::create(&desc_path)?;
    desc_file.write_all(&[255, 254])?;
    for utf16 in desc.encode_utf16() {
        desc_file.write_all(&(bincode::serialize(&utf16).unwrap())).unwrap();
    }

    let icon_xdb_path = PathBuf::from(format!("{}GOG_Mod\\{}\\Icon.xdb", profile.data_path, &icon_path));
    if !icon_xdb_path.exists() {
        let icon_xdb = base_cfg.generic_icon_128.as_ref().unwrap();
        let icon_dds = base_cfg.generic_icon_dds.as_ref().unwrap();
        std::fs::copy(icon_xdb, &icon_xdb_path)?;
        std::fs::copy(icon_dds, icon_xdb_path.to_str().unwrap().replace(".xdb", ".dds"))?;
    }

    let universe_pak_path = PathBuf::from(format!("{}Universe_mod.pak", profile.data_path));
    let temp_pak_path = PathBuf::from(format!("{}Universe_mod_temp.pak", profile.data_path));
    let temp_file = std::fs::File::create(&temp_pak_path)?;
    let old_file = std::fs::File::open(&universe_pak_path)?;
    let mut old_archive = zip::ZipArchive::new(old_file).unwrap();
    let mut new_archive = zip::ZipWriter::new(temp_file);
    let file_options = FileOptions::default().last_modified_time(zip::DateTime::from_date_and_time(2107, 12, 31, 23, 59, 59).unwrap());
    for i in 0..old_archive.len() {
        let mut entry = old_archive.by_index(i).unwrap();
        if entry.name() == "types.xml" {
            let updated_types = process_types_xml(&mut entry, game_id.clone())?;
            new_archive.start_file("types.xml", file_options)?;
            new_archive.write_all(&updated_types)?;
        } else if entry.name() == "GameMechanics/RefTables/UndividedSpells.xdb" {
            let updated_spells = process_undivided_spells(&mut entry, name.clone(), game_id.clone())?;
            new_archive.start_file("GameMechanics/RefTables/UndividedSpells.xdb", file_options)?;
            new_archive.write_all(&updated_spells)?;
        } else {
            new_archive.raw_copy_file(entry)?;
        }
    }
    std::fs::rename(temp_pak_path, universe_pak_path)?;
    let spell_xdb_dir = PathBuf::from(format!("{}GOG_Mod\\GameMechanics\\Spell\\GOG\\{}\\", &profile.data_path, &name));
    if !spell_xdb_dir.exists() {
        std::fs::create_dir_all(&spell_xdb_dir)?;
    }
    let spell_xdb_path = spell_xdb_dir.join("Spell.xdb");
    let mut xdb_file = std::fs::File::create(&spell_xdb_path)?;

    let created_spell = scaner_service.add_spell(CreateSpellPayload {
        desc,
        desc_txt: desc_path.to_str().unwrap().replace(&profile.texts_path, "").replace("\\", "/"),
        name: name.clone(),
        name_txt: name_path.to_str().unwrap().replace(&profile.texts_path, "").replace("\\", "/"),
        icon_xdb: format!("{}#xpointer(/Texture)", icon_xdb_path.to_str().unwrap().replace(&format!("{}GOG_Mod", &profile.data_path), "")).replace("\\", "/"),
        game_id,
        school,
        xdb_path: spell_xdb_path.to_str().unwrap().replace(&format!("{}GOG_Mod", &profile.data_path), "").replace("\\", "/")
    }).await?;
    let mut output: Vec<u8> = Vec::new();
    let mut writer = Writer::new_with_indent(&mut output, b' ', 4);
        writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;
    let shared: SpellShared = created_spell.clone().into();
    writer.write_serializable("Spell", &shared)?;
    xdb_file.write_all(&output)?;

    Ok(created_spell)
}

#[derive(Debug, Serialize, Deserialize)]
struct EnumEntries {
    #[serde(rename = "Item")]
    pub entries: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct SharedClassEntry {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Value")]
    pub value: usize
}

#[derive(Debug, Serialize, Deserialize)]
struct SharedClassEntries {
    #[serde(rename = "Item")]
    pub entries: Vec<SharedClassEntry>
}

fn process_types_xml(zip_file: &mut ZipFile<'_>, game_id: String) -> Result<Vec<u8>, Error> {
    let mut types_xml_string = String::new();
    zip_file.read_to_string(&mut types_xml_string)?;

    let mut output: Vec<u8> = Vec::new();
    let mut writer = Writer::new_with_indent(&mut output, b' ', 4);
    let mut reader = Reader::from_str(&types_xml_string);
    let reader_config = reader.config_mut();
    reader_config.expand_empty_elements = true;
    reader_config.trim_text(true);

    let mut buf: Vec<u8> = Vec::new();
    let mut path_stack = vec![];
    let mut spell_table_found = false;
    let mut spell_shared_classes_found = false;
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                let actual_tag = std::str::from_utf8(e.name().as_ref())?.to_string();
                path_stack.push(actual_tag);
                let xpath = path_stack.iter().join("/");
                // println!("XPath: {}", &xpath);
                match xpath.as_str() {
                    "Base/Tables/Item/dbid/XPointer" => {
                        path_stack.pop();
                        // println!("Found pointer");
                        let end = e.to_end().into_owned();
                        let text = reader.read_text(end.name())?.to_string();
                        // println!("With text: {}", &text);
                        if text == "/GameMechanics/RefTables/UndividedSpells.xdb#xpointer(/Table_Spell_SpellID)" {
                            spell_table_found = true;
                        }
                        writer.create_element("XPointer").write_text_content(BytesText::new(&text))?;
                    },
                    "Base/Tables/Item/EnumEntries" => {
                        path_stack.pop();
                        let end = e.to_end().into_owned();
                        let text = reader.read_text(end.name())?.to_string();
                        let mut entries: EnumEntries = quick_xml::de::from_str(&format!("<EnumEntries>{}</EnumEntries>", &text))?;
                        if spell_table_found {
                            spell_table_found = false;
                            entries.entries.push(game_id.clone());
                        }
                        writer.write_serializable("EnumEntries", &entries)?;
                    }
                    "Base/SharedClasses/Item/__ServerPtr" => {
                        path_stack.pop();
                        let end = e.to_end().into_owned();
                        let text = reader.read_text(end.name())?.to_string();
                        // println!("With text: {}", &text);
                        if text == "8d02a80a" {
                            spell_shared_classes_found = true;
                        }
                        writer.create_element("__ServerPtr").write_text_content(BytesText::new(&text))?;
                    }
                    "Base/SharedClasses/Item/Entries" => {
                        path_stack.pop();
                        let end = e.to_end().into_owned();
                        let text = reader.read_text(end.name())?.to_string();
                        let mut entries: SharedClassEntries = quick_xml::de::from_str(&format!("<Entries>{}</Entries>", &text))?;
                        if spell_shared_classes_found {
                            spell_shared_classes_found = false;
                            entries.entries.push(SharedClassEntry { name: game_id.clone(), value: entries.entries.len() });
                        }
                        writer.write_serializable("Entries", &entries)?;
                    }
                    _=> {
                        let mut elem = BytesStart::new(str::from_utf8(e.name().0).unwrap());
                        elem.extend_attributes(e.attributes().map(|attr| attr.unwrap()));
                        writer.write_event(Event::Start(elem)).unwrap();
                    }
                }
            }
            Ok(Event::Text(e)) => {
                writer.write_event(Event::Text(e)).unwrap();
            },
            Ok(Event::End(e)) => {
                path_stack.pop();
                let elem = BytesEnd::new(str::from_utf8(e.name().0).unwrap());
                writer.write_event(Event::End(elem)).unwrap();
            },
            _ => ()
        }
        buf.clear();
    }
    Ok(output)
}

fn process_undivided_spells(zip_file: &mut ZipFile<'_>, name: String, game_id: String) -> Result<Vec<u8>, Error> {
    let mut spells_string = String::new();
    zip_file.read_to_string(&mut spells_string)?;

    let mut output: Vec<u8> = Vec::new();
    let mut writer = Writer::new_with_indent(&mut output, b' ', 4);
    let mut reader = Reader::from_str(&spells_string);
    let reader_config = reader.config_mut();
    reader_config.expand_empty_elements = true;
    reader_config.trim_text(true);

    let mut buf: Vec<u8> = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                let actual_tag = std::str::from_utf8(e.name().as_ref())?.to_string();
                match actual_tag.as_str() {
                    "objects" => {
                        let end = e.to_end().into_owned();
                        let text = reader.read_text(end.name())?.to_string();
                        let mut file_objects: FileObjects = quick_xml::de::from_str(&format!("<objects>{}</objects>", &text))?;
                        file_objects.objects.push(FileObject { 
                            ID: game_id.clone(),
                            Obj: Some(FileRef {
                                href: Some(format!("/GameMechanics/Spell/GOG/{}/Spell.xdb#xpointer(/Spell)", &name))
                            })
                        });
                        writer.write_serializable("objects", &file_objects)?;
                    }
                    _=> {
                        let mut elem = BytesStart::new(str::from_utf8(e.name().0).unwrap());
                        elem.extend_attributes(e.attributes().map(|attr| attr.unwrap()));
                        writer.write_event(Event::Start(elem)).unwrap();
                    }
                }
            }
            Ok(Event::Text(e)) => {
                writer.write_event(Event::Text(e)).unwrap();
            },
            Ok(Event::End(e)) => {
                let elem = BytesEnd::new(str::from_utf8(e.name().0).unwrap());
                writer.write_event(Event::End(elem)).unwrap();
            },
            _ => ()
        }
        buf.clear();
    }

    Ok(output)
}

#[tauri::command]
pub async fn save_spell_xdb(
    app_manager: State<'_, LocalAppManager>,
    scaner_service: State<'_, ScanerService>,
    id: i32
) -> Result<(), Error> {
    let profile = app_manager.current_profile_data.read().await;
    if let Some(model) = scaner_service.get_spell(id).await? {
        let name_path = PathBuf::from(format!("{}{}", &profile.texts_path, &model.name_txt));
        let desc_path = PathBuf::from(format!("{}{}", &profile.texts_path, &model.desc_txt));

        let mut name_file = std::fs::File::create(&name_path)?;
        name_file.write_all(&[255, 254])?;
        for utf16 in model.name.encode_utf16() {
            name_file.write_all(&(bincode::serialize(&utf16).unwrap())).unwrap();
        }

        let mut desc_file = std::fs::File::create(&desc_path)?;
        desc_file.write_all(&[255, 254])?;
        for utf16 in model.desc.encode_utf16() {
            desc_file.write_all(&(bincode::serialize(&utf16).unwrap())).unwrap();
        }

        let spell_xdb_path = PathBuf::from(format!("{}GOG_Mod\\{}", &profile.data_path, &model.xdb_path));
        let mut xdb_file = std::fs::File::create(&spell_xdb_path)?;
        let mut output: Vec<u8> = Vec::new();
        let mut writer = Writer::new_with_indent(&mut output, b' ', 4);
            writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;
        let shared: SpellShared = model.into();
        writer.write_serializable("Spell", &shared)?;
        xdb_file.write_all(&output)?;
    }
    Ok(())
}