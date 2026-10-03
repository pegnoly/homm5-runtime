use std::collections::HashMap;
use std::fmt::Display;
use itertools::Itertools;
use serde::Deserialize;

#[derive(Debug)]
pub struct DialogStepModel<'a> {
    pub speakers: Vec<&'a str>,
    pub labels: Vec<&'a str>
}

impl<'a> Display for DialogStepModel<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ speakers = {{{}}}, labels = {{{}}} }}", self.speakers.iter().map(|s| {
            if let Ok(number) = s.parse::<i32>() {
                number.to_string()
            } else {
                format!("\"{}\"", s)
            }
        }).collect_vec().join(", "), self.labels.iter().map(|l| format!("\"{}\"", l)).collect_vec().join(", "))
    }
}
