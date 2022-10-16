extern crate core;

use std::env;

use backend::file_utils::create_file;
use memmap2::Mmap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, Error, Write};

// Split a big xml file into small xml files
fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let xml_input_file = args[1].as_str();
    let folder_path = args[2].as_str();
    let chunk_size_str = args[3].as_str();
    let tag = args[4].as_str();
    let chunk_size: u32 = String::from(chunk_size_str).parse().unwrap();

    const HEAD_LINE: &'static str = "<?xml version=\"1.0\" encoding=\"utf-8\"?>";
    let START_TAG: String = format!("<{}>", tag);
    let END_TAG: String = format!("</{}>", tag);

    let xml_part0 = format!("{}_1.xml", tag);
    create_file(folder_path, &xml_part0);

    let mut output_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(format!("{}/{}", folder_path, xml_part0))
        .unwrap();

    let file = File::open(format!("./stackdata/{}", xml_input_file))?;
    let map = unsafe { Mmap::map(&file)? };
    let mut count = 0;
    let mut part = 1;

    // Read line by line
    for s in map.split(|x| *x == 10) {
        //10 is new line character
        let line = std::str::from_utf8(&s).unwrap();
        let _ = writeln!(output_file, "{}", line.to_string());
        count += 1;

        if count % chunk_size == 0 {
            // Append end tag
            let _ = writeln!(output_file, "{}", END_TAG);

            // Create new file & writing new head
            part += 1;
            let new_filename = format!("{}_{}.xml", tag, part);
            create_file(folder_path, new_filename.as_str());
            output_file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(format!("{}/{}", folder_path, new_filename))
                .unwrap();
            let _ = writeln!(output_file, "{}", HEAD_LINE);
            let _ = writeln!(output_file, "{}", START_TAG);
        }
    }
    Ok(())
}
