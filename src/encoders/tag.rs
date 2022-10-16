use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub struct TagRow {
    id: u64,
    excerpt_post_id: String,
    wiki_post_id: String,
    tag_name: String,
    count: String,
}
