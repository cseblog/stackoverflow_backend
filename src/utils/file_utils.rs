// use futures::AsyncReadExt;
// use memmap2::Mmap;
// use serde::{Deserialize, Serialize};
use serde_json::{to_string, Result};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

// pub fn to_json<T> (post_links: StackData<T>, file_name: &str) {
//     let links = post_links.rows;
//     write_to_file(to_string(&links).unwrap(), String::from(file_name));
// }

pub fn create_file(path: &str, filename: &str) -> File {
    let path_str = format!("{}/{}", path, filename);
    let path = Path::new(&path_str);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    return file;
}

pub fn write_to_file(data_str: String, filename: String) {
    let mut file = create_file("./data/", &filename[..]);
    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(data_str.as_bytes()) {
        Err(why) => panic!("couldn't write to {}", why),
        Ok(_) => println!("successfully wrote to {}", filename),
    }
}

pub fn read_xml_file(file_path: String) -> String {
    // Create a path to the desired file
    let path = Path::new(&file_path);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => return s,
    }
}
