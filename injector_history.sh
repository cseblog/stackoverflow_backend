#! /bin/sh
search_dir="./stackdata/split/posthistory"
type="post_history"
for file in "$search_dir"/*
do      
        echo "Injecting $file..."
        cargo run --bin injector "$file" $type
done



