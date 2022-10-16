use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct StackData<T> {
    #[serde(rename = "row", default)]
    pub rows: Vec<T>,
}
