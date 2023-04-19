use std::path::PathBuf;

use clap::{arg, command, value_parser, ArgAction, Command};

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
    let matches = command!()
        .name("bitsquid_unbundler")
        .version("0.1.0")
        .author("Alias")
        .about("Extracts assets from bitsquid compiled bundles.")
        .arg(arg!(-i --indir <INPUT_DIRECTORY> "The input directory containing the bitsquid compiled assets.").required(false).value_parser(value_parser!(PathBuf)))
        .arg(arg!(-o --outdir <OUTPUT_DIRECTORY> "The output directory which the extracted files shall be written to.").required(false).value_parser(value_parser!(PathBuf)))
        .arg(arg!(-fmww --findmww).required(false))
        .get_matches();

}
