use std::io;
use std::fs::File;
use std::ffi::OsStr;
use std::path::Path;
use std::io::prelude::*;

pub fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut contents = String::new();
    let mut f = File::open(filename)?;
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn get_extension_from_filename(filename: &str) -> Option<&str> {
    let res = Path::new(filename).extension().and_then(OsStr::to_str);
    res
}
