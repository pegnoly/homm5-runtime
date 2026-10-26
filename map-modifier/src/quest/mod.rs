use itertools::Itertools;
use std::{fs::File, io::Write, path::PathBuf};
use convert_case::{Case, Casing};
use editor_tools::prelude::{QuestProgressType, QuestUpdateSources};
use homm5_types::{common::FileRef, quest::Quest};
use crate::error::MapModifierError;
use crate::GenerateBoilerplate;

#[derive(Debug)]
pub struct QuestProgress {
    pub progress_type: QuestProgressType,
    pub concatenate: bool
}

// frontend send this when user wants to create a new quest with given params
#[derive(Default)]
pub struct QuestCreationRequest {
    campaign_number: i32,
    mission_number: i32,
    path: PathBuf,
    name: String,
    script_name: String,
    desc: String,
    progresses: Vec<QuestProgress>,
    secondary: bool,
    initialy_active: bool,
    update_sources: QuestUpdateSources
}

impl QuestCreationRequest {

    pub fn new(path: PathBuf, script_name: String) -> Self {
        QuestCreationRequest { script_name, path, ..Default::default()}
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn with_desc(mut self, desc: String) -> Self {
        self.desc = desc;
        self
    }

    pub fn with_progresses(mut self, progresses: Vec<QuestProgress>) -> Self {
        self.progresses = progresses;
        self
    }

    pub fn with_mission_data(mut self, campaign: i32, mission: i32) -> Self {
        self.campaign_number = campaign;
        self.mission_number = mission;
        self
    }

    pub fn secondary(mut self, is_secondary: bool) -> Self {
        self.secondary = is_secondary;
        self
    }

    pub fn initialy_active(mut self, is_active: bool) -> Self {
        self.initialy_active = is_active;
        self
    }

    pub fn with_update_sources(mut self, update_sources: QuestUpdateSources) -> Self {
        self.update_sources = update_sources;
        self
    }

    fn generate_name(&self, quest: &mut Quest, base_texts_dir: &String, map_local_data: &String) {
        let mut file = File::create(format!("{base_texts_dir}name.txt")).unwrap();
        file.write_all(&[255, 254]).unwrap(); // byte-order mask for homm encoding
        for utf16 in self.name.encode_utf16() {
            file.write_all(&(bincode::serialize(&utf16).unwrap())).unwrap();
        }

        let local_file_name = format!("{map_local_data}\\name.txt").replace("\\", "/");

        quest.caption_file_ref = FileRef {href: Some(local_file_name)};
    }

    fn generate_desc(&self, quest: &mut Quest, base_texts_dir: &String, map_local_data: &String) {
        let mut file = File::create(format!("{base_texts_dir}desc.txt")).unwrap();
        file.write_all(&[255, 254]).unwrap(); // byte-order mask for homm encoding
        for utf16 in self.desc.encode_utf16() {
            file.write_all(&(bincode::serialize(&utf16).unwrap())).unwrap();
        }

        let local_file_name = format!("{map_local_data}\\desc.txt").replace("\\", "/");

        quest.description_file_ref = FileRef {href: Some(local_file_name)};
    }

    fn generate_progresses(&self, quest: &mut Quest, progresses_texts_dir: &String, map_local_data: &String) -> Result<(), MapModifierError> {
        for file in std::fs::read_dir(progresses_texts_dir)? {
            let file = file?;
            let path = file.path();
            if path.is_file() {
                std::fs::remove_file(path)?;
            }
        }

        let mut previous_progresses = String::new();

        quest.progress_comments_file_ref.items = Some(vec![]);

        let mut count = 0;
        for progress in &self.progresses {
            match &progress.progress_type {
                QuestProgressType::Default(text) => {
                    self.create_progress(count, text, progresses_texts_dir, map_local_data, &mut previous_progresses, quest)?;
                    count += 1;
                    if progress.concatenate {
                        previous_progresses += &format!("{}\n\n", text);
                    }
                },
                QuestProgressType::OneOf(data) => {
                    let mut generated_text = String::new() ;
                    for value in data.start_value..data.count + 1 {
                        generated_text = data.text.replace("[current_value]", &value.to_string())
                            .replace("[values_count]", &data.count.to_string());
                        self.create_progress(count, &generated_text, progresses_texts_dir, map_local_data, &mut previous_progresses, quest)?;
                        count += 1;
                    }
                    if progress.concatenate {
                        previous_progresses += &format!("{}\n\n", generated_text);
                    }
                }
            }
        }
        Ok(())
    }
    fn create_progress(
        &self,
        number: i32,
        text: &String,
        dir: &String,
        map_local_dir: &String,
        previous: &mut String,
        quest: &mut Quest
    ) -> Result<(), MapModifierError> {
        let mut file = File::create(format!("{dir}{number}.txt"))?;
        file.write_all(&[255, 254])?;
        let current_progress = format!("<color=grey>{}<color=white>{}", previous, text);
        for utf16 in current_progress.encode_utf16() {
            file.write_all(&(bincode::serialize(&utf16)?))?;
        }

        let local_file_name = format!("{}\\Progress\\{}.txt", map_local_dir, number).replace("\\", "/");

        quest.progress_comments_file_ref.items.as_mut().unwrap().push(FileRef {href: Some(local_file_name)});
        Ok(())
    }

    fn create_update_source_enum(
        &self,
    ) -> Result<String, MapModifierError> {
        let p_name = self.script_name.to_case(Case::Pascal);
        let result = format!("
---@enum {}UpdateSource
{}UpdateSource = {{
    {}
}}
", &p_name,
&p_name,
self.update_sources.sources.iter().enumerate()
                     .map(|(index, source)| format!("{} = {},{}", &source.source, index + 1,
                                                    if source.comment.is_empty() {
                                                        String::new()
                                                    } else {
                                                        format!("-- {}", source.comment)
                                                    }))
            .collect::<Vec<String>>().join("\n")
        );

        Ok(result)
    }

    fn create_update_source_context(
        &self,
    ) -> Result<String, MapModifierError> {
        let p_name = self.script_name.to_case(Case::Pascal);
        let result = format!("
---@class __{}UpdateContext: QuestUpdateContext
---@field source {}UpdateSource
{}
", &p_name, &p_name, self.update_sources.sources.iter().flat_map(|v| &v.variables)
.map(|v| format!("---@field {} {}", &v.name.to_case(Case::Snake).to_lowercase(), &v.variable_type.to_string().to_lowercase()))
.collect_vec().join("\n"));
        Ok(result)
    }
}

pub struct QuestBoilerplateHelper {
    pub mod_path: String,
    pub map_data_path: String,
    pub texts_path: String
}

impl GenerateBoilerplate for QuestCreationRequest {
    type Output = Quest;
    type Additional = QuestBoilerplateHelper;

    fn generate(&self, additional_data: Option<&QuestBoilerplateHelper>) -> Result<Quest, MapModifierError> {
        let mut quest = Quest {
            name: Some(self.script_name.clone()),
            is_hidden: false,
            is_initialy_active: self.initialy_active,
            ..Default::default()
        };

        let helper_data = additional_data.unwrap();
        // represents path as game sees it
        let local_quest_dir = self.path.to_str().unwrap().replace(&helper_data.mod_path, "");

        let quest_texts_base = format!("{}{}\\", &helper_data.texts_path, &local_quest_dir);
        let texts_dir = PathBuf::from(format!("{}Texts\\", &quest_texts_base));

        let dialogs_texts_dir = PathBuf::from(format!("{}Dialogs\\", &quest_texts_base));
        let texts_path = self.path.join("Texts\\");
        let dialogs_path = self.path.join("Dialogs\\");
        let progresses_texts_dir = PathBuf::from(format!("{}Progress\\", &quest_texts_base));

        let map_local_path = self.path.to_str().unwrap().replace(&helper_data.map_data_path, "");

        if !texts_dir.exists() {
            std::fs::create_dir_all(&texts_dir)?;
        }
        if !dialogs_texts_dir.exists() {
            std::fs::create_dir_all(&dialogs_texts_dir)?;
        }
        if !dialogs_path.exists() {
            std::fs::create_dir(&dialogs_path)?;
        }
        if !progresses_texts_dir.exists() {
            std::fs::create_dir_all(&progresses_texts_dir)?;
        }

        self.generate_name(&mut quest, &quest_texts_base, &map_local_path);
        self.generate_desc(&mut quest, &quest_texts_base, &map_local_path);
        self.generate_progresses(&mut quest, &progresses_texts_dir.to_string_lossy().to_string(), &map_local_path)?;

        let script_path = self.path.join("script.lua");
        let p_name = self.script_name.to_case(Case::Pascal);
        if !script_path.exists() {
        let script_boilerplate = format!("
{}
---@class __{}StartContext: QuestStartContext

{}
---@class __{}CompleteContext: QuestCompleteContext

---@class __{}FailContext: QuestFailContext

---@class __{}: QuestBase
---@field Start fun(self, context: __{}StartContext)
---@field Update fun(self, context: __{}UpdateContext)
---@field Complete fun(self, context: __{}CompleteContext)
---@field Fail fun(self, context: __{}FailContext)

---@return __{}
function __{}()
    local _quest = QuestBase({{
        id = \"{}\",
        name_path = \"{}\",
        text_path = \"{}\"
    }}) --[[@as __{}]]

    function _quest:Init()

    end

    ---@param context __{}StartContext
    function _quest:Start(context)

    end

    ---@param context __{}UpdateContext
    function _quest:Update(context)

    end

    ---@param context __{}CompleteContext
    function _quest:Complete(context)

    end

    ---@param context __{}FailContext
    function _quest:Fail(context)

    end

    return _quest
end

{} = __{}()

__end_import()",
            self.create_update_source_enum()?,
            &p_name,
            self.create_update_source_context()?,
            &p_name,
            &p_name,
            &p_name,
            &p_name,
            &p_name,
            &p_name,
            &p_name,
            &p_name,
            &p_name,
            self.script_name,
            self.path.join("name.txt").to_str().unwrap().replace(&additional_data.unwrap().mod_path, "").replace("\\", "/"),
            texts_path.to_str().unwrap().replace(&additional_data.unwrap().mod_path, "").replace("\\", "/"),
            &p_name,
            &p_name,
            &p_name,
            &p_name,
            &p_name,
            &p_name,
            &p_name
        );

            let mut script_file = File::create(self.path.join("script.lua"))?;
            script_file.write_all(script_boilerplate.as_bytes())?;
        }
    
        Ok(quest)
    }
}

pub fn write_quest_text_file(file: &mut File, text: String) {
    file.write_all(&[255, 254]).unwrap(); 
    for utf16 in text.encode_utf16() {
        file.write_all(&(bincode::serialize(&utf16).unwrap())).unwrap();
    }
}