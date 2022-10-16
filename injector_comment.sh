#! /bin/sh
search_dir="./stackdata/split/comments"
type="comments"
for file in "$search_dir"/*
do      
        echo "Injecting $file..."
        cargo run --bin injector "$file" $type
done



