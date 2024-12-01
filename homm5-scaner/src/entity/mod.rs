pub mod hero;
pub mod creature;
pub mod art;
pub mod spell;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use homm5_types::common::FileRef;
use crate::pak::FileStructure;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct FileObject {
    pub ID: String,
    pub Obj: Option<FileRef>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileObjects {
    #[serde(rename = "Item")]
    pub objects: Vec<FileObject>
}

pub trait CollectFiles {
    fn collect(&self, files: &HashMap<String, FileStructure>, collected_files: &mut Vec<(String, FileStructure)>);
}

pub trait Scan<T> {
    fn scan(&mut self, file_key: &String, entity: &String, files: &HashMap<String, FileStructure>) -> Option<Box<dyn Output<ID = T>>>;
    fn get_id(&self) -> Option<T>;
}

pub trait Output {
    type ID;
    fn to_lua(&self, id: Option<Self::ID>) -> String;
    fn to_json(&self) -> String;
}

pub struct ScanProcessor<T> {
    pub table_name: String,
    pub output_file_name: String,
    pub collector: Box<dyn CollectFiles>,
    pub scaner: Box<dyn Scan<T>>
}

impl<T> ScanProcessor<T> {
    pub fn new(table: String, output: String, collector: Box<dyn CollectFiles>, processor: Box<dyn Scan<T>>) -> Self {
        ScanProcessor { 
            table_name: table, 
            output_file_name: output, 
            collector: collector,
            scaner: processor 
        }
    }
}

impl<T> ScanProcessor<T> {
    pub fn run(&mut self, files: &HashMap<String, FileStructure>) -> (String, String) {
        let mut actual_files = vec![];
        self.collector.collect(files, &mut actual_files);
        println!("files collected: {:?}", &actual_files);
        let mut output_string = format!("{} = {{\n", &self.table_name);
        let mut json_string = String::from("[");
        for file in actual_files {
            let scanned_file = self.scaner.scan(&file.0, &file.1.content, files);
            let id = self.scaner.get_id();
            match scanned_file {
                Some(actual_file) => {
                    output_string += &actual_file.to_lua(id);
                    json_string += &format!("{},\n", actual_file.to_json());
                },
                None => {}
            }
        }
        output_string.trim_end_matches(",").to_string();
        output_string.push('}');
        json_string = json_string.trim_end_matches(",").to_owned();
        (output_string, json_string)
    }
}

pub fn configure_path(path: Option<&String>, file_key: &String, files: &HashMap<String, FileStructure>) -> String {
    match path {
        Some(actual_path) => {
            let actual_path = actual_path.trim_start_matches("/").to_lowercase();
            println!("actual path is {}", &actual_path);
            if files.contains_key(&actual_path) == false {
                println!("and it not in files");
                let actual_name = file_key.rsplit_once("/").unwrap().0.to_string() + &format!("/{}", &actual_path);
                //println!("name was {}, key {}, actual name {}", actual_path, file_key, &actual_name);
                actual_name
            }
            else {
                actual_path
            }
        }
        None => {
            String::new()
        }
    }
}