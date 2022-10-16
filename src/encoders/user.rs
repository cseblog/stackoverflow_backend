use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub struct UserRow {
    id: u64,
    account_id: String,
    reputation: String,
    views: String,
    down_votes: String,
    up_votes: String,
    display_name: String,
    location: String,
    profile_image_url: String,
    website_url: String,
    about_me: String,
    creation_date: String,
    last_access_date: String,
}
