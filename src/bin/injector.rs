extern crate core;

use backend::file_utils::read_xml_file;
use backend::*;
use futures::executor::block_on;
use lazy_static::lazy_static;
use meilisearch_sdk::client::*;
use meilisearch_sdk::errors::Error;
// use meilisearch_sdk::settings::Settings;
use meilisearch_sdk::tasks::Task;
use quick_xml::de::from_str;
use quick_xml::DeError;
use std::env;
use std::thread::sleep;
use std::time::Duration;

lazy_static! {
    static ref CLIENT: Client = Client::new("http://localhost:7700", "abc");
}

fn tag_read_file(file_path: &str) -> StackData<TagRow> {
    let s = read_xml_file(String::from(file_path));
    let result: Result<StackData<TagRow>, DeError> = from_str(&*s);
    let data: StackData<TagRow> = result.ok().unwrap();
    return data;
}

fn comments_read_file(file_path: &str) -> StackData<CommentRow> {
    let s = read_xml_file(String::from(file_path));
    let result: Result<StackData<CommentRow>, DeError> = from_str(&*s);
    let data: StackData<CommentRow> = result.ok().unwrap();
    return data;
}

fn history_read_file(file_path: &str) -> StackData<PostHistoryRow> {
    let s = read_xml_file(String::from(file_path));
    let result: Result<StackData<PostHistoryRow>, DeError> = from_str(&*s);
    let data: StackData<PostHistoryRow> = result.ok().unwrap();
    return data;
}

fn post_read_file(file_path: &str) -> StackData<PostRow> {
    let s = read_xml_file(String::from(file_path));
    let result: Result<StackData<PostRow>, DeError> = from_str(&*s);
    let data: StackData<PostRow> = result.ok().unwrap();
    return data;
}

fn vote_read_file(file_path: &str) -> StackData<VoteRow> {
    let s = read_xml_file(String::from(file_path));
    let result: Result<StackData<VoteRow>, DeError> = from_str(&*s);
    let data: StackData<VoteRow> = result.ok().unwrap();
    return data;
}

//TODO Move to Injector
// async fn build_comment_index() {
//     println!("Build comment index...");

//     let settings = Settings::new().with_filterable_attributes(["Id", "PostId"]);
//     let result = CLIENT
//         .index("comments")
//         .set_settings(&settings)
//         .await
//         .unwrap()
//         .wait_for_completion(&CLIENT, None, None)
//         .await
//         .unwrap();

//     if result.is_failure() {
//         panic!(
//             "Encountered an error while setting settings for index: {:?}",
//             result.unwrap_failure()
//         );
//     }
// }

//TODO: Move to Injector
// async fn build_vote_index() {
//     println!("Build vote index...");
//     let settings = Settings::new().with_filterable_attributes(["Id", "PostId"]);

//     let result = CLIENT
//         .index("votes")
//         .set_settings(&settings)
//         .await
//         .unwrap()
//         .wait_for_completion(&CLIENT, None, Option::from(Duration::from_secs(120)))
//         .await
//         .unwrap();

//     if result.is_failure() {
//         panic!(
//             "Encountered an error while setting settings for index: {:?}",
//             result.unwrap_failure()
//         );
//     }
// }

//TODO: Move to Injector
// async fn build_post_index() {
//     println!("Build vote index...");
//     let settings = Settings::new().with_filterable_attributes(["Id", "ParentId"]);

//     let result = CLIENT
//         .index("posts")
//         .set_settings(&settings)
//         .await
//         .unwrap()
//         .wait_for_completion(&CLIENT, None, Option::from(Duration::from_secs(120)))
//         .await
//         .unwrap();

//     if result.is_failure() {
//         panic!(
//             "Encountered an error while setting settings for index: {:?}",
//             result.unwrap_failure()
//         );
//     }
// }

// Go through all XML files, convert it into Json files
// Inject into Json files into Meilisearch
fn main() {
    println!("----Injector -----");
    let args: Vec<String> = env::args().collect();
    let xml_file = args[1].as_str();
    let collection_name = args[2].as_str();

    let host = "http://localhost:7700";
    let api_key = "abc";

    // Create a client (without sending any request so that can't fail)
    let client = Client::new(host, api_key);
    match collection_name {
        "tags" => {
            //tag
            let data = tag_read_file(xml_file);
            block_on(async move {
                // An index is where the documents are stored.
                let index = client.index(collection_name);
                let rows = data.rows;
                let chunks: Vec<_> = rows.chunks(100000).collect();
                for ch in chunks.iter() {
                    println!("{:?}", ch.get(1));
                    let _ = index
                        .add_documents_in_batches(&ch, Some(100000), None)
                        .await;
                    sleep(Duration::from_secs(20));
                }
            });
        }
        "comments" => {
            //comment
            let data = comments_read_file(xml_file);
            block_on(async move {
                // An index is where the documents are stored.
                let index = client.index(collection_name);
                let rows = data.rows;
                let chunks: Vec<_> = rows.chunks(100000).collect();
                for ch in chunks.iter() {
                    println!("{:?}", ch.get(1));
                    let _ = index
                        .add_documents_in_batches(&ch, Some(100000), None)
                        .await;
                    sleep(Duration::from_secs(10));
                }
            });
        }
        "post_history" => {
            // history
            let data = history_read_file(xml_file);
            block_on(async move {
                // An index is where the documents are stored.
                let index = client.index(collection_name);
                let rows = data.rows;
                let chunks: Vec<_> = rows.chunks(100000).collect();
                for ch in chunks.iter() {
                    println!("{:?}", ch.get(1));
                    let _ = index
                        .add_documents_in_batches(&ch, Some(100000), None)
                        .await;
                    sleep(Duration::from_secs(10));
                }
            });
        }
        "post" => {
            // posts
            let data = post_read_file(xml_file);
            block_on(async move {
                // An index is where the documents are stored.
                let index = client.index(collection_name);
                let rows = data.rows;
                let chunks: Vec<_> = rows.chunks(100000).collect();
                let mut total = 0;
                for ch in chunks.iter() {
                    total += ch.len();
                    println!("======== ADDING.... {} total: {}", ch.len(), total);
                    let _ = index
                        .add_documents_in_batches(&ch, Some(100000), None)
                        .await;
                    sleep(Duration::from_secs(10));
                }
            });
        }
        "votes" => {
            // Votes
            let data = vote_read_file(xml_file);
            block_on(async move {
                // An index is where the documents are stored.
                let index = client.index(collection_name);
                let rows = data.rows;
                let chunks: Vec<_> = rows.chunks(100000).collect();
                for ch in chunks.iter() {
                    println!("{:?}", ch.get(1));
                    let _ = index
                        .add_documents_in_batches(&ch, Some(100000), None)
                        .await;
                    sleep(Duration::from_secs(10));
                }
            });
        }
        _ => {
            println!("Invalid collection name!")
        }
    }
}
