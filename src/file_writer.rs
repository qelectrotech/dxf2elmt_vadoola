extern crate tempfile;

use anyhow::Context;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use tempfile::tempfile;

pub fn create_file(verbose_output: bool, _info: bool, file_name: &Path) -> File {
    let old_file_name = file_name.to_string_lossy();
    
    let mut file_name = PathBuf::from(file_name);
    file_name.set_extension("elmt");

    let friendly_file_name = file_name.to_string_lossy();
    let mut out_file = tempfile().context("Could not create temporary file");
    if !verbose_output {
        //out_file = File::create(format!("{}.elmt", &file_name[0..file_name.len() - 4]))
        out_file = File::create(&file_name)
            .context("Could not create output file");
        println!(
            "{friendly_file_name} was created... \nNow converting {old_file_name}...",
        );
    }

    out_file.context("Could not return output file").unwrap()
}

pub fn verbose_print(mut out_file: std::fs::File) -> File {
    out_file
        .seek(SeekFrom::Start(0))
        .context("Could not find beginning of output file")
        .unwrap();
    let mut v_contents = String::new();
    out_file
        .read_to_string(&mut v_contents)
        .context("Could not read output file to a string")
        .unwrap();
    print!("{v_contents}");

    out_file
}
