use std::{io::Write, marker::PhantomData, path::PathBuf};

use homm5_types::{common::FileRef, quest::Quest, Homm5Type};
use quick_xml::se;

use crate::GenerateBoilerplate;


pub fn test_convert(quest: Quest) {
    let s = quick_xml::se::to_string(&quest).unwrap();
    println!("Quest string is: {}", s);
}

#[derive(Default)]
pub struct QuestProgress {
    pub number: u32,
    pub text: String,
    pub concatenate: bool
}

// frontend send this when user wants to create a new quest with given params
#[derive(Default)]
pub struct QuestCreationRequest {
    campaign_number: u8,
    mission_number: u8,
    path: PathBuf,
    name: String,
    script_name: String,
    desc: String,
    progresses: Vec<QuestProgress>,
    secondary: bool,
    initialy_active: bool,
}

impl QuestCreationRequest {

    pub fn new(path: PathBuf, script_name: String) -> Self {
        let mut request = QuestCreationRequest::default();
        request.script_name = script_name;
        request.path = path;
        request
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

    pub fn with_mission_data(mut self, campaign: u8, mission: u8) -> Self {
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

    fn generate_name(&self, quest: &mut Quest, map_data_path: &String) {
        let file_name = self.path.join("name.txt");
        let mut file = std::fs::File::create(&file_name).unwrap();
        file.write(&[255, 254]).unwrap(); // byte-order mask for homm encoding
        for utf16 in self.name.encode_utf16() {
            file.write(&(bincode::serialize(&utf16).unwrap())).unwrap();
        }

        let local_file_name = file_name.to_str().unwrap().replace(map_data_path, "").replace("\\", "/");

        quest.caption_file_ref = FileRef {href: Some(local_file_name)};
    }

    fn generate_desc(&self, quest: &mut Quest, map_data_path: &String) {
        let file_name = self.path.join("desc.txt");
        let mut file = std::fs::File::create(&file_name).unwrap();
        file.write(&[255, 254]).unwrap(); // byte-order mask for homm encoding
        for utf16 in self.desc.encode_utf16() {
            file.write(&(bincode::serialize(&utf16).unwrap())).unwrap();
        }

        let local_file_name = file_name.to_str().unwrap().replace(map_data_path, "").replace("\\", "/");

        quest.description_file_ref = FileRef {href: Some(local_file_name)};
    }

    fn generate_progresses(&self, directory: &PathBuf, quest: &mut Quest, map_data_path: &String) {

        let mut previous_progresses = String::new();

        quest.progress_comments_file_ref.items = Some(vec![]);

        for progress in &self.progresses {
            let file_name = directory.join(format!("{}.txt", progress.number));
            let mut file = std::fs::File::create(&file_name).unwrap();
            file.write(&[255, 254]).unwrap(); // byte-order mask for homm encoding

            let current_progress = format!("<color=grey>{}<color=white>{}", &previous_progresses, &progress.text);

            for utf16 in current_progress.encode_utf16() {
                file.write(&(bincode::serialize(&utf16).unwrap())).unwrap();
            }

            if progress.concatenate {
                previous_progresses += &format!("{}\n\n", progress.text);
            }

            let local_file_name = file_name.to_str().unwrap().replace(map_data_path, "").replace("\\", "/");

            quest.progress_comments_file_ref.items.as_mut().unwrap().push(FileRef {href: Some(local_file_name)});
        }
    }
}

pub struct QuestBoilerplateHelper {
    pub mod_path: String,
    pub map_data_path: String
}

impl GenerateBoilerplate for QuestCreationRequest {
    type Output = Quest;
    type Additional = QuestBoilerplateHelper;

    fn generate(&self, additional_data: Option<&QuestBoilerplateHelper>) -> Quest {
        let mut quest = Quest::default();
        quest.name = self.script_name.clone();
        quest.is_hidden = false;
        quest.is_initialy_active = self.initialy_active;

        let texts_path = self.path.join("Texts\\");
        let dialogs_path = self.path.join("Dialogs\\");
        let progress_path = self.path.join("Progress\\");

        std::fs::create_dir(&texts_path).unwrap();
        std::fs::create_dir(&dialogs_path).unwrap();
        std::fs::create_dir(&progress_path).unwrap();

        self.generate_name(&mut quest, &additional_data.unwrap().map_data_path);
        self.generate_desc(&mut quest, &additional_data.unwrap().map_data_path);
        self.generate_progresses(&progress_path, &mut quest, &additional_data.unwrap().map_data_path);

        let script_boilerplate = format!("
c{}m{}_{} = {{
    name = \"{}\",
    path = {{
        text = \"{}\",
        dialog = \"{}\"
    }},

    Init = 
    function ()
        Quest.Names[\"{}\"] = \"{}\"
    end
}}", 
            self.campaign_number, 
            self.mission_number, 
            self.script_name.to_lowercase(), 
            self.script_name, 
            texts_path.to_str().unwrap().replace(&additional_data.unwrap().mod_path, "").replace("\\", "/"),
            dialogs_path.to_str().unwrap().replace(&additional_data.unwrap().mod_path, "").replace("\\", "/"),
            self.script_name,
            self.path.join("name.txt").to_str().unwrap().replace(&additional_data.unwrap().mod_path, "").replace("\\", "/")
        );

        let mut script_file = std::fs::File::create(self.path.join("script.lua")).unwrap();
        script_file.write_all(script_boilerplate.as_bytes()).unwrap();
        quest
    }
}