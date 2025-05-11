pub mod art;
pub mod creature;
pub mod hero;
pub mod spell;

use crate::{models, pak::FileStructure};
use homm5_types::common::FileRef;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct FileObject {
    pub ID: String,
    pub Obj: Option<FileRef>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileObjects {
    #[serde(rename = "Item")]
    pub objects: Vec<FileObject>,
}

pub trait CollectFiles {
    fn collect(
        &self,
        files: &HashMap<String, FileStructure>,
        collected_files: &mut Vec<(String, FileStructure)>,
    );
}

pub trait Scan {
    type Output;
    fn scan(
        &mut self,
        file_key: &String,
        entity: &String,
        files: &HashMap<String, FileStructure>,
    ) -> Self::Output;
}

pub trait Output {
    type Input;
    fn output_single(&self, object: Self::Input);
    fn finish_output(&self);
}

pub struct CreatureOutput;

impl Output for CreatureOutput { 
    type Input = models::creature::Model;

    fn output_single(&self, object: Self::Input) {}

    fn finish_output(&self) {}
}

pub struct ScanProcessor<T> {
    pub collector: Box<dyn CollectFiles>,
    pub scaner: Box<dyn Scan<Output = T>>,
    pub writer: Box<dyn Output<Input = T>>
}

impl<T> ScanProcessor<T> {
    pub fn new(
        collector: Box<dyn CollectFiles>,
        scaner: Box<dyn Scan<Output = T>>,
        writer: Box<dyn Output<Input = T>>
    ) -> Self {
        ScanProcessor {
            collector,
            scaner,
            writer
        }
    }
}

impl<T> ScanProcessor<T> {
    pub fn run(&mut self, files: &HashMap<String, FileStructure>) {
        let mut actual_files = vec![];
        self.collector.collect(files, &mut actual_files);
        for file in actual_files {
            let scanned_file = self.scaner.scan(&file.0, &file.1.content, files);
            self.writer.output_single(scanned_file);
        }
        self.writer.finish_output();
    }
}

pub fn configure_path(
    path: Option<&String>,
    file_key: &String,
    files: &HashMap<String, FileStructure>,
) -> String {
    match path {
        Some(actual_path) => {
            let actual_path = actual_path.trim_start_matches("/").to_lowercase();
            //println!("actual path is {}", &actual_path);
            if files.contains_key(&actual_path) == false {
                //println!("and it not in files");
                if let Some(actual_name) = file_key.rsplit_once("/") {
                    let final_name = actual_name.0.to_string() + &format!("/{}", &actual_path);
                    final_name
                } else {
                    String::new()
                }
            } else {
                actual_path
            }
        }
        None => String::new(),
    }
}
