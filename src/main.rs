mod unbundler;
mod file_writer;
pub mod byte_stream;
pub mod unbundled_file;
//----------------------\\
use std::ffi::OsString;
use std::path::PathBuf;
use std::fs::{ReadDir, DirEntry};
use std::fs;
//----------------------\\
use clap::{arg, command, value_parser, ArgMatches};
use registry::{Hive, Security};
//----------------------\\
use crate::file_writer::FileWriter;
use crate::unbundler::{Unbundler, UnbundlerError};
use crate::unbundled_file::UnbundledFile;


const DIRECTORY_ERROR_MSG: &str = "Could not create a directory for the output. This may be due to insuffient permissions for creating a directory at this location.";

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let matches = command!()
        .name("bitsquid_unbundler")
        .version("1.0.0")
        .author("Alias")
        .about("Extracts assets from bitsquid compiled bundles.")
        .arg(arg!(-i --indir <INPUT_DIRECTORY> "[Optional] The input directory containing the bitsquid compiled assets.\n\tDefaults to searching the Steam directory for Magicka: Wizard Wars' data_win32_bundled directory.").required(false).value_parser(value_parser!(OsString)))
        .arg(arg!(-o --outdir <OUTPUT_DIRECTORY> "[Optional] The output directory which the extracted files shall be written to.\n\tDefaults to creating a directory in the present working directory of the executable for output.").required(false).value_parser(value_parser!(OsString)))
        .get_matches();

    let input_directory = read_input_directory(&matches);
    let output_directory = create_output_directory(&matches);

    unbundle_files(input_directory, output_directory);
}

fn create_output_directory(matches: &ArgMatches) -> PathBuf {
    match matches.get_one::<OsString>("outdir") {
        Some(outdir) => {
            match outdir.to_str() {
                Some(str) => {
                    fs::create_dir(str).expect(DIRECTORY_ERROR_MSG);
                    PathBuf::from(str)
                },
                None => create_output_directory_in_pwd()
            }
        },
        None => create_output_directory_in_pwd()
    }
}

fn create_output_directory_in_pwd() -> PathBuf {
    println!("Creating output directory in the present working directory for the executable since the -o <OUTPUT_DIRECTORY> argument either was not supplied or was invalid.");
    let mut path = std::env::current_dir()
        .expect("Could not get the present working directory for the executable. This may be due to insuffient permissions for this directory.");

    path.push("unbundled");
    fs::create_dir(&path).unwrap();

    path
}

fn read_input_directory(matches: &ArgMatches) -> ReadDir {
    if let Some(indir) = matches.get_one::<OsString>("indir") {
        let error = "The input directory provided to -i could not be read. This could be due to insufficient permissions, a path that doesn't exist, or the path is not a directory.";
        fs::read_dir(indir).expect(error)
    } else {
        read_input_dir_from_registry()
    }
}

fn read_input_dir_from_registry() -> ReadDir {
    let error = "Could not find the Steam directory to locate the Magicka: Wizard Wars data_win32_bundled directory";
    let data_win32_path = r"\steamapps\common\MagickaWizardWars\data_win32_bundled";

    let steam_dir = Hive::LocalMachine.open(r"SOFTWARE\WOW6432Node\Valve\Steam", Security::Read).expect(error);
    let bundle_directory = OsString::from(format!("{}{}", steam_dir.value("InstallPath").unwrap().to_string(), data_win32_path));

    fs::read_dir(bundle_directory).expect(error)
}

fn unbundle_files(bundle_dir: ReadDir, write_dir: PathBuf) {
    for bundle_path_result in bundle_dir {
        match bundle_path_result  {
            Ok(bundle_path) => try_unbundle_file(bundle_path, &write_dir),
            Err(e) => println!("An unexpected error occurred when reading a bundle file. Error: {}\nSkipping file.", e),
        }
    }
}

fn try_unbundle_file(bundle_path: DirEntry, write_dir: &PathBuf) {
    let bundle_pathbuf = bundle_path.path();
    
    match bundle_pathbuf.extension() {
        Some(ext) => match ext.to_str().unwrap() {
            "stream" => return,
            "ini" => return,
            "data" => return,
            _ => (),
        },
        None => (),
    }

    //This should be a safe unwrap since error handling has already been done for bundle_path.
    let path = bundle_pathbuf
        .as_os_str()
        .to_str()
        .unwrap(); 

    match unbundle_file(path) {
        Ok(files) => {
            let mut out_path = write_dir.clone();
            out_path.push(bundle_path.file_name());
            fs::create_dir(&out_path).unwrap();

            //This should be a safe unwrap since error handling is already done for the output directory.
            FileWriter::write_files(out_path.to_str().unwrap(), files);
        },
        Err(e) => println!("An error occurred while attempting to unbundle {}\n Error: {:?}", path, e),
    }
}

fn unbundle_file(path: &str) -> Result<Vec<UnbundledFile>, UnbundlerError> {
    match Unbundler::new(path) {
        Ok(mut unbundler) => unbundler.unbundle_file(),
        Err(_) => Err(UnbundlerError::IOError),
    }
}