#! /bin/sh
search_dir="./stackdata/split/votes"
type="votes"
for file in "$search_dir"/*
do      
        echo "Injecting $file..."
        cargo run --bin injector "$search_dir/$file" $type
done



