use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub struct CommentRow {
    id: u64,
    post_id: String,
    user_id: String,
    score: String,
    content_license: String,
    user_display_name: String,
    text: String,
    creation_date: String,
}
