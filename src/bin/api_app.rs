extern crate core;
use actix_web::{post, web, App, HttpServer, Responder};
use backend::*;
use lazy_static::lazy_static;
use meilisearch_sdk::client::*;
use serde::Deserialize;
use serde::Serialize;
use std::string::String;

lazy_static! {
    static ref CLIENT: Client = Client::new("http://localhost:7700", "abc");
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub struct Question {
    vote_count: u64,
    post: PostRow,
    answers: Vec<Answer>,
    comments: Vec<CommentRow>,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub struct Answer {
    vote_count: u64,
    post: PostRow,
    comments: Vec<CommentRow>,
}

async fn search_post(query: &str) -> Vec<PostRow> {
    //TODO: return 100 latest post
    //TODO: return 100 hostest post by votes
    let query_results = CLIENT
        .index("posts")
        .search()
        .with_query(query)
        .execute::<PostRow>()
        .await
        .unwrap()
        .hits;

    let mut vec: Vec<PostRow> = Vec::new();
    if !query_results.is_empty() {
        for rows in query_results {
            let row = rows.result;
            if row.parent_id == "" {
                vec.push(row);
            }
        }
    }
    println!("Search hits: {} results", vec.len());
    return vec;
}

async fn get_post_by_id(id: u64) -> Option<Question> {
    let query = format!("Id = {}", id);
    let search_result = CLIENT
        .index("posts")
        .search()
        .with_query("")
        .with_filter(&*query)
        .execute::<PostRow>()
        .await
        .unwrap();

    let hits = search_result.hits;

    for rows in hits {
        let row = rows.result;
        let id = row.id;
        let votes = get_vote_by_post_id(id).await;
        let comments = get_comments_by_post_id(id).await;
        let answers = get_child_posts_by_parent_id(id).await;
        return Some(Question {
            vote_count: votes,
            post: row,
            comments,
            answers,
        });
    }
    return None;
}

async fn get_comments_by_post_id(id: u64) -> Vec<CommentRow> {
    let query = format!("PostId = {}", id);
    let search_result = CLIENT
        .index("comments")
        .search()
        .with_query("")
        .with_filter(&*query)
        .execute::<CommentRow>()
        .await
        .unwrap();

    let hits = search_result.hits;
    let mut vec = vec![];
    for row in hits {
        vec.push(row.result)
    }
    return vec;
}

async fn get_child_posts_by_parent_id(id: u64) -> Vec<Answer> {
    let query = format!("ParentId = {}", id);
    let search_result = CLIENT
        .index("posts")
        .search()
        .with_query("")
        .with_filter(&*query)
        .execute::<PostRow>()
        .await
        .unwrap();

    let hits = search_result.hits;
    let mut vec: Vec<Answer> = Vec::new();
    for row in hits {
        let post = row.result;
        let vote_count = get_vote_by_post_id(post.id).await;
        let comments = get_comments_by_post_id(id).await;
        vec.push(Answer {
            vote_count,
            post,
            comments,
        })
    }
    return vec;
}

async fn get_vote_by_post_id(id: u64) -> u64 {
    let query = format!("PostId = {}", id);
    let search_result = CLIENT
        .index("votes")
        .search()
        .with_query("")
        .with_filter(&*query)
        .execute::<CommentRow>()
        .await
        .unwrap();
    return search_result.nb_hits as u64;
}

#[post("/search")]
async fn search(req_body: String) -> impl Responder {
    let post_list = search_post(req_body.as_str()).await;
    return web::Json(post_list);
}

#[post("/search/{id}")]
async fn get_post(id: web::Path<u64>) -> impl Responder {
    let post_id = id.into_inner();
    let post = get_post_by_id(post_id).await;
    return web::Json(post);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Stackoverlfow server...");

    HttpServer::new(|| App::new().service(search).service(get_post))
        .bind(("localhost", 9990))?
        .run()
        .await
}
