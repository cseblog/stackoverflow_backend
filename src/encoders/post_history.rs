use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub struct PostHistoryRow {
    id: String,
    post_id: String,
    user_id: String,
    post_history_type_id: String,
    user_display_name: String,
    content_license: String,
    #[serde(rename = "RevisionGUID", default)]
    revision_guid: String,
    text: String,
    comment: String,
    creation_date: String,
}
