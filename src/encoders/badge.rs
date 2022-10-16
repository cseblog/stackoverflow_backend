use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub struct BadgeRow {
    id: u64,
    user_id: String,
    class: String,
    name: String,
    tag_based: String,
    date: String,
}
