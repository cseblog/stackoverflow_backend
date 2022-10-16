use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub struct VoteRow {
    id: u64,
    post_id: String,
    vote_type_id: String,
    creation_date: String,
    bounty_amount: String,
    user_id: String,
}
