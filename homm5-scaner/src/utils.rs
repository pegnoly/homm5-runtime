use std::collections::HashMap;

use crate::pak::FileStructure;

pub fn configure_path(
    path: Option<&String>,
    file_key: &str,
    files: &HashMap<String, FileStructure>,
) -> String {
    match path {
        Some(actual_path) => {
            let actual_path = actual_path.trim_start_matches("/").to_lowercase();
            //println!("actual path is {}", &actual_path);
            if !files.contains_key(&actual_path) {
                //println!("and it not in files");
                if let Some(actual_name) = file_key.rsplit_once("/") {
                    actual_name.0.to_string() + &format!("/{}", &actual_path)
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

pub trait IntoSeaormCondition {
    fn into_condition(&self) -> sea_orm::Condition;
}