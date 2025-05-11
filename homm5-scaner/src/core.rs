use crate::{error::ScanerError, pak::FileStructure};
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
    ) -> Result<(), ScanerError>;
}

pub trait Scan {
    type Output;

    fn scan(
        &mut self,
        file_key: &String,
        entity: &String,
        files: &HashMap<String, FileStructure>,
    ) -> Result<Option<Self::Output>, ScanerError>;
}

pub trait Output {
    type Input;

    fn output_single(&mut self, object: Self::Input) -> Result<(), ScanerError>;
    async fn finish_output(&self) -> Result<(), ScanerError>;
}

pub struct ScanProcessor<T, C: CollectFiles, S: Scan<Output = T>, W: Output<Input = T>> {
    collector: C,
    scaner: S,
    writer: W
}

impl<T, C: CollectFiles, S: Scan<Output = T>, W: Output<Input = T>> ScanProcessor<T, C, S, W> {
    pub fn new(
        collector: C,
        scaner: S,
        writer: W
    ) -> Self {
        ScanProcessor {
            collector,
            scaner,
            writer
        }
    }

    pub async fn run(&mut self, files: &HashMap<String, FileStructure>) -> Result<(), ScanerError> {
        let mut actual_files = vec![];
        self.collector.collect(files, &mut actual_files)?;
        for file in actual_files {
            if let Some(scanned_file) = self.scaner.scan(&file.0, &file.1.content, files)? {
                self.writer.output_single(scanned_file)?;
            }
        }
        self.writer.finish_output().await?;
        Ok(())
    }
}