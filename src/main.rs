use std::env;

mod unbundler;
mod file_writer;

pub mod byte_stream;
pub mod unbundled_file;

/*
    Use cases:
        Put in bundled folder. execute. it dumps unpacked stuff into a sub-directory.
        Given argument -find will look for the data_win32_bundled on its own then treat it as if only input was given.
        Given argument -find and output directory as arg, find the data_win32_bundled and dump to output dir.
        Given input directory as arg. Create subdirectory wherever it is located and dump unpacked stuff into it.
        Given input and output directories as args, read from input dir, dump into output dir.
    */

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_dir: String;
    let output_dir: String;
    if args.contains(&"-find".to_string()) {
        
    }
}
