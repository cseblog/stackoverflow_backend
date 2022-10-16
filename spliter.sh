#! /bin/sh

echo "Splitting Comments.xml..."
cargo run --bin spliter "Comments.xml" "./stackdata/split" 1000000 "comments"

echo "Splitting Badges.xml..."
cargo run --bin spliter "Badges.xml" "./stackdata/split" 1000000 "badges"

echo "Spliting PostHistory.xml..."
cargo run --bin spliter "PostHistory.xml" "./stackdata/split" 1000000 "posthistory"

echo "Splitting Posts.xml..."
cargo run --bin spliter "Posts.xml" "./stackdata/split" 1000000 "posts"


echo "Splitting Tags.xml..."
cargo run --bin spliter "Tags.xml" "./stackdata/split" 1000000 "tags"

