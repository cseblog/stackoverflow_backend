use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub struct LinkRow {
    id: u64,
    related_post_id: String,
    post_id: String,
    link_type_id: String,
    creation_date: String,
}
