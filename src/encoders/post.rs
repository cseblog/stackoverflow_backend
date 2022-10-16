use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub struct PostRow {
    pub id: u64,
    pub owner_user_id: String,
    pub last_editor_user_id: String,
    pub post_type_id: String,
    pub accepted_answer_id: String,
    pub score: String,
    pub parent_id: String,
    pub view_count: String,
    pub answer_count: String,
    pub comment_count: String,
    pub owner_display_name: String,
    pub last_editor_display_name: String,
    pub title: String,
    pub tags: String,
    pub content_license: String,
    pub body: String,
    pub favorite_count: String,
    pub creation_date: String,
    pub community_owned_date: String,
    pub closed_date: String,
    pub last_edit_date: String,
    pub last_activity_date: String,
}
