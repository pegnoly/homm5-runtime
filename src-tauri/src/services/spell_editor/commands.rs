use std::{io::{Read, Write}, path::{Path, PathBuf}};

use homm5_scaner::prelude::{MagicSchoolType, ScanerError, ScanerService, SpellDBModel};
use itertools::Itertools;
use quick_xml::{Reader, Writer, events::{BytesEnd, BytesStart, BytesText, Event}};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, State};
use tauri_plugin_dialog::DialogExt;
use zip::read::ZipFile;

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
) -> Result<(), Error> {
    let profile = app_manager.current_profile_data.read().await;
    let texts_dir = PathBuf::from(format!("{}{}\\{}\\", &profile.texts_path, &texts_path, &name));
    if !texts_dir.exists()  {
        std::fs::create_dir_all(&texts_dir)?;
    }
    let name_path = texts_dir.join("name.txt");
    let desc_path = texts_dir.join("desc.txt");

    let universe_pak_path = PathBuf::from(format!("{}Universe_mod.pak", profile.data_path));
    let temp_pak_path = PathBuf::from(format!("{}Universe_mod_temp.pak", profile.data_path));
    let temp_file = std::fs::File::create(&temp_pak_path)?;
    let old_file = std::fs::File::open(&universe_pak_path)?;
    let mut old_archive = zip::ZipArchive::new(old_file).unwrap();
    let mut new_archive = zip::ZipWriter::new(temp_file);

    for i in 0..old_archive.len() {
        let mut entry = old_archive.by_index(i).unwrap();
        if entry.name() == "types.xml" {
            process_types_xml(&mut entry, game_id.clone())?;
        } else if entry.name() == "GameMechanics/RefTables/UndividedSpells.xdb" {

        } 
        new_archive.raw_copy_file(entry).unwrap();
    }

    // let mut output: Vec<u8> = Vec::new();
    // let mut writer = Writer::new_with_indent(&mut output, b' ', 4);
    // writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;
    // writer.create_element("Table_DBArtifact_ArtifactEffect").write_inner_content(|w| {
    //     w.write_serializable("objects", &table).unwrap();
    //     Ok(())
    // })?;
    // new_archive.start_file("GameMechanics/RefTables/Artifacts.xdb", FileOptions::default()).unwrap();
    // new_archive.write_all(&output)?;
    // new_archive.finish().unwrap();
    std::fs::rename(temp_pak_path, universe_pak_path)?;

    Ok(())
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

fn process_types_xml(zip_file: &mut ZipFile<'_>, game_id: String) -> Result<(), Error> {
    println!("Started");
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
    let mut file = std::fs::File::create("D:\\Homm5Dev\\types_test.xml")?;
    file.write_all(&output)?;
    Ok(())
}