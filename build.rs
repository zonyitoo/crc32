

use std::io;

#[path = "src/crc32gen.rs"]
mod crc32gen;

fn main() {
    let outfile_name = "src/crc32tables.rs";

    println!("generating crc tables");

    let crc_tables = crc32gen::make_crc_table();
    let s = crc32gen::write_tables(&crc_tables);

    let outpath = Path::new(outfile_name);

    let outfile = io::File::create(&outpath);
    let mut outwr = io::BufferedWriter::new(outfile);

    outwr.write_str(s.as_slice()).unwrap();
}
