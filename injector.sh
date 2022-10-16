#! /bin/sh
search_dir="./stackdata/split/tags"
type="tags"
id=1
for file in "$search_dir"/*
do      
        echo "Injecting $file..."
        cargo run --bin injector "$file" $type $id
done
