use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn write_file(file: &mut File, buffer: &str) {
    file.write_all(buffer.as_bytes())
        .expect("unable to write to file");
}