use std::{io::Write, path::PathBuf};

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
    pub text: String
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
    initialy_active: bool
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

    pub fn generate_name(&self, quest: &mut Quest) {
        let file_name = self.path.join("name.txt");
        let mut file = std::fs::File::create(&file_name).unwrap();
        file.write(&[255, 254]).unwrap(); // byte-order mask for homm encoding
        for utf16 in self.name.encode_utf16() {
            file.write(&(bincode::serialize(&utf16).unwrap())).unwrap();
        }
        quest.caption_file_ref = FileRef {href: Some(file_name.to_str().unwrap().to_string())};
    }

    pub fn generate_desc(&self, quest: &mut Quest) {
        let file_name = self.path.join("desc.txt");
        let mut file = std::fs::File::create(&file_name).unwrap();
        file.write(&[255, 254]).unwrap(); // byte-order mask for homm encoding
        for utf16 in self.desc.encode_utf16() {
            file.write(&(bincode::serialize(&utf16).unwrap())).unwrap();
        }
        quest.description_file_ref = FileRef {href: Some(file_name.to_str().unwrap().to_string())};
    }

    pub fn generate_progresses(&self, directory: &PathBuf, quest: &mut Quest) {

        let mut previous_progresses = String::new();

        for progress in &self.progresses {
            let file_name = directory.join(format!("{}.txt", progress.number));
            let mut file = std::fs::File::create(&file_name).unwrap();
            file.write(&[255, 254]).unwrap(); // byte-order mask for homm encoding

            let current_progress = format!("<color=grey>{}<color=white>{}", &previous_progresses, &progress.text);

            for utf16 in current_progress.encode_utf16() {
                file.write(&(bincode::serialize(&utf16).unwrap())).unwrap();
            }

            previous_progresses += &format!("{}\n\n", progress.text);

            quest.progress_comments_file_ref.push(FileRef {href: Some(file_name.to_str().unwrap().to_string())});
        }
    }
}


impl GenerateBoilerplate for QuestCreationRequest {
    type Output = Quest;

    fn generate(&self) -> Quest {
        let mut quest = Quest::default();
        quest.name = self.script_name.clone();

        let texts_path = self.path.join("Texts\\");
        let dialogs_path = self.path.join("Dialogs\\");
        let progress_path = self.path.join("Progress\\");

        std::fs::create_dir(&texts_path).unwrap();
        std::fs::create_dir(&dialogs_path).unwrap();
        std::fs::create_dir(&progress_path).unwrap();

        self.generate_name(&mut quest);
        self.generate_desc(&mut quest);
        self.generate_progresses(&progress_path, &mut quest);

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
            texts_path.to_str().unwrap().replace("\\", "/"),
            dialogs_path.to_str().unwrap().replace("\\", "/"),
            self.script_name,
            self.path.join("name.txt").to_str().unwrap().replace("\\", "/")
        );

        let mut script_file = std::fs::File::create(self.path.join("script.lua")).unwrap();
        script_file.write_all(script_boilerplate.as_bytes()).unwrap();
        quest
    }
}