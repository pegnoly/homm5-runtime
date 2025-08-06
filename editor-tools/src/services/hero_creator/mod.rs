use std::{fs::File, io::{BufWriter, Write}, path::PathBuf};
use ddsfile::Dds;
use homm5_scaner::prelude::Town;
use image::{imageops::FilterType, ImageReader};
use quick_xml::{events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event}, Reader, Writer};
use widestring::Utf16String;

use crate::error::EditorToolsError;

pub fn process_png(input_path: &str, output_path: &str) -> Result<(), EditorToolsError> {
    let image = ImageReader::open(input_path)?.decode()?;
    let resized = image.resize_exact(128, 128, FilterType::Lanczos3);

    let mut dds = Dds::new_d3d(
        ddsfile::NewD3dParams { 
            height: resized.height(), 
            width: resized.width(), 
            depth: None, 
            format: ddsfile::D3DFormat::A8R8G8B8, 
            mipmap_levels: None, 
            caps2: None,
        },
    )?;

    let rgba = resized.to_rgba8();

    let mipmap = dds.get_mut_data(0)?;
    mipmap.copy_from_slice(&rgba.into_raw());

    let mut file = BufWriter::new(File::create(output_path)?);
    dds.write(&mut file)?;

    Ok(())
}

pub fn process_files(
    generic_hero_path: &PathBuf,
    generic_icon_path: &PathBuf,
    generic_icon_dds: &PathBuf,
    mod_path: String,
    hero_town: Town, 
    hero_script_name: String,
    hero_actual_name: String,
) -> Result<(), EditorToolsError> {
    let mut output_hero_file: Vec<u8> = Vec::new();
    let mut writer = Writer::new_with_indent(&mut output_hero_file, b' ', 4);

    let hero_string = std::fs::read_to_string(&generic_hero_path).unwrap();

    let mut reader = Reader::from_str(&hero_string);
    let reader_config = reader.config_mut();
    reader_config.expand_empty_elements = true;
    reader_config.trim_text(true);

    let mut buf: Vec<u8> = Vec::new();

    
    let town_path = match hero_town {
        Town::TownAcademy => "Academy",
        Town::TownDungeon => "Dungeon",
        Town::TownFortress => "Dwarves",
        Town::TownHeaven => "Haven",
        Town::TownInferno => "Inferno",
        Town::TownNecromancy => "Necropolis",
        Town::TownPreserve => "Preserve",
        Town::TownNoType => "Neutral",
        Town::TownStronghold => "Orcs"
    };

    let name_file_dir = PathBuf::from(format!("{}Text\\Game\\Heroes\\Persons\\{}\\{}\\", &mod_path, town_path, &hero_script_name));
    let icon_file_dir = PathBuf::from(format!("{}Textures\\Icons\\Heroes\\{}\\{}\\", &mod_path, town_path, &hero_script_name));
    if !name_file_dir.exists() {
        std::fs::create_dir_all(&name_file_dir)?;
    }
    if !icon_file_dir.exists() {
        std::fs::create_dir_all(&icon_file_dir)?;
    }
    let name_file = name_file_dir.join("Name.txt");
    let bio_file = name_file_dir.join("Bio.txt");
    let icon_file = icon_file_dir.join("Icon.xdb");
    let icon_dds = icon_file_dir.join("Icon.dds");

    writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                // gets actual name of tag
                let actual_tag = std::str::from_utf8(e.name().as_ref())?.to_string();
                match actual_tag.as_str() {
                    "InternalName" => {
                        let end = e.to_end().into_owned();
                        reader.read_to_end(end.name())?;
                        writer.create_element("InternalName").write_text_content(BytesText::new(&hero_script_name))?;
                    },
                    "NameFileRef" => {
                        let end = e.to_end().into_owned();
                        reader.read_to_end(end.name())?;
                        writer.create_element("NameFileRef")
                            .with_attribute(("href", 
                                format!("/{}", name_file
                                    .to_str()
                                    .ok_or(EditorToolsError::Default)?
                                    .replace(&mod_path, "")
                                    .replace("\\", "/")
                                    .as_str()
                                ).as_str()))
                            .write_empty()?;
                    },
                    "BiographyFileRef" => {
                        let end = e.to_end().into_owned();
                        reader.read_to_end(end.name())?;
                        writer.create_element("BiographyFileRef")
                            .with_attribute(("href", 
                                format!("/{}", bio_file
                                    .to_str()
                                    .ok_or(EditorToolsError::Default)?
                                    .replace(&mod_path, "")
                                    .replace("\\", "/")
                                    .as_str()
                                ).as_str()))
                            .write_empty()?;
                    },
                    "Icon128" => {
                        let end = e.to_end().into_owned();
                        reader.read_to_end(end.name())?;
                        writer.create_element("Icon128")
                            .with_attribute(("href", 
                                format!("/{}#xpointer(/Texture)", icon_file
                                    .to_str()
                                    .ok_or(EditorToolsError::Default)?
                                    .replace(&mod_path, "")
                                    .replace("\\", "/")
                                    .as_str()
                                ).as_str()))
                            .write_empty()?;
                    },
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

    let town_path = match hero_town {
        Town::TownAcademy => "Academy",
        Town::TownDungeon => "Dungeon",
        Town::TownFortress => "Dwarves",
        Town::TownHeaven => "Haven",
        Town::TownInferno => "Inferno",
        Town::TownNecromancy => "Necropolis",
        Town::TownPreserve => "Preserve",
        Town::TownNoType => "Neutral",
        Town::TownStronghold => "Orcs"
    };

    let xdb_file_dir = PathBuf::from(format!("{}MapObjects\\{}\\Heroes\\{}\\", &mod_path, town_path, &hero_script_name));
    if !xdb_file_dir.exists() {
        std::fs::create_dir_all(&xdb_file_dir)?;
    }

    let mut xdb_file = std::fs::File::create(xdb_file_dir.join("Hero.(AdvMapHeroShared).xdb".to_string()))?;
    xdb_file.write_all(&output_hero_file)?;

    std::fs::copy(generic_icon_path, icon_file)?;
    std::fs::copy(generic_icon_dds, icon_dds)?;

    let mut actual_name_file = std::fs::File::create(&name_file)?;
    let utf16_data = Utf16String::from_str(&hero_actual_name);

    actual_name_file.write_all(&[0xFF, 0xFE])?;
    
    // Write the UTF-16 LE bytes
    actual_name_file.write_all(unsafe {
        std::slice::from_raw_parts(
            utf16_data.as_ptr() as *const u8,
            utf16_data.len() * 2
        )
    })?;

    Ok(())
}