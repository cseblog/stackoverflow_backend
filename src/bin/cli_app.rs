extern crate core;
use actix_web::{post, web, App, HttpResponse, Responder};
use backend::*;
use futures::executor::block_on;
use lazy_static::lazy_static;
use meilisearch_sdk::client::*;
use std::io::stdin;
use std::string::String;

lazy_static! {
    static ref CLIENT: Client = Client::new("http://localhost:7700", "abc");
}

async fn search_post(query: &str) {
    let query_results = CLIENT
        .index("posts")
        .search()
        .with_query(query)
        .execute::<PostRow>()
        .await
        .unwrap()
        .hits;

    if query_results.is_empty() {
        println!("no results...");
    } else {
        for rows in query_results {
            let display = rows.result;
            println!("{}", format_args!("{:?}", display));
        }
    }
}

async fn get_post_by_id(id: u64) {
    let query = format!("Id = {}", id);
    let search_result = CLIENT
        .index("posts")
        .search()
        .with_query("")
        .with_filter(&*query)
        .execute::<PostRow>()
        .await
        .unwrap();

    let r = search_result.hits;
    for r in r.iter() {
        let id = r.result.id;
        //
        let votes = get_vote_by_post_id(id).await;
        println!("============ POST =============");
        println!("{:?}", r.result);
        println!("Votes: {:?}", votes);
        println!("===============================");
        get_comments_by_post_id(id);
        get_child_posts_by_parent_id(id);
    }
}

async fn get_comments_by_post_id(id: u64) {
    let query = format!("PostId = {}", id);
    let search_result = CLIENT
        .index("comments")
        .search()
        .with_query("")
        .with_filter(&*query)
        .execute::<CommentRow>()
        .await
        .unwrap();

    let r = search_result.hits;
    for r in r.iter() {
        println!("========== COMMENT ===================");
        println!("{:?}", r.result);
        println!("======================================");
    }
}

async fn get_child_posts_by_parent_id(id: u64) {
    let query = format!("ParentId = {}", id);
    let search_result = CLIENT
        .index("posts")
        .search()
        .with_query("")
        .with_filter(&*query)
        .execute::<PostRow>()
        .await
        .unwrap();

    let r = search_result.hits;
    for r in r.iter() {
        let id = r.result.id;
        get_vote_by_post_id(id);
        get_comments_by_post_id(id);
        println!("============== ANSWER ==========");
        println!("{:?}", r.result);
        println!("================================");
    }
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
    HttpResponse::Ok().body(req_body)
}

#[post("/search/{id}")]
async fn get_post(id: web::Path<u64>) -> impl Responder {
    HttpResponse::Ok().body("test")
}

fn main() {
    println!("Search demo...");
    block_on(async move {
        // enter in search queries or quit
        loop {
            println!("Enter a search query or type \"q\" or \"quit\" to quit:");
            let mut input_string = String::new();

            stdin()
                .read_line(&mut input_string)
                .expect("Failed to read line");

            let args: Vec<&str> = input_string.split(" ").collect();
            let cm = args.get(0).unwrap();
            let id = args.get(1).unwrap();
            let input = String::from(*cm);

            match input.trim() {
                "quit" | "q" | "" => {
                    println!("exiting...");
                    break;
                }
                "get_p" => {
                    get_post_by_id(id.trim().parse().unwrap()).await;
                }
                "get_pp" => {
                    get_child_posts_by_parent_id(id.parse().unwrap()).await;
                }
                "get_c" => {
                    get_comments_by_post_id(id.parse().unwrap()).await;
                }
                "get_v" => {
                    let votes = get_vote_by_post_id(id.parse().unwrap()).await;
                    println!("PostID: {}, votes: {}", 28018, votes);
                }
                _ => {
                    search_post(input.trim()).await;
                }
            }
        }
    });
}
