#![feature(fs, io, path)]

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

#[path = "src/crc32gen.rs"]
mod crc32gen;

fn main() {
    let outfile_name = "src/crc32tables.rs";

    println!("generating crc tables");

    let crc_tables = crc32gen::make_crc_table();
    let s = crc32gen::write_tables(&crc_tables);

    let outpath = Path::new(outfile_name);

    let outfile = File::create(&outpath).unwrap();
    let mut outwr = BufWriter::new(outfile);

    outwr.write_all(s.as_bytes()).unwrap();
}
