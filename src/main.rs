mod unbundler;
mod file_writer;
pub mod byte_stream;

use std::env;

use unbundler::UnbundledFile;

use crate::unbundler::Unbundler;
use crate::file_writer::FileWriter;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let input_dir = "D:\\Disk Drive Downloads\\Steam\\steamapps\\common\\MagickaWizardWars\\data_win32_bundled\\00c4323d97062055";
    let output_dir = "C:\\Users\\Nathan\\Desktop\\test";
    let mut unbundler = Unbundler::new(input_dir);
    let files = unbundler.unbundle_files();
    FileWriter::write_files(output_dir, &files)
    //let file_path = format!("{}\\{:#x}.{}", output_dir, unbundled.path, FileWriter::lookup_extension_name(unbundled.extension));
    //FileWriter::write_file(&file_path, &unbundled);
}
