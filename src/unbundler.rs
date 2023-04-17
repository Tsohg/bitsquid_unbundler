// u32: bitsquid header -> 0xf0000004
// u32: total size of uncompressed files
// u32: reserved space or padding.
// 
// For each zlib chunk until the total size has been met:
//  u32: chunk length. next byte is the start of a zlib file.
//  [i->len]: zlib chunk.

use std::fs;
use std::io::Write;
use crate::byte_stream::ByteStream;
use flate2::write::ZlibDecoder;

#[derive(Clone)]
pub struct UnbundledFile {
    pub path: u64,
    pub extension: u64,
    pub data: Vec<u8>,
}

pub struct Unbundler {
    compressed_stream: ByteStream,
}

impl<'a> Unbundler {
    pub fn new(compressed_file_path: &'a str) -> Unbundler {
        Unbundler {
            compressed_stream: ByteStream::new(fs::read(compressed_file_path).unwrap()),
        }
    }

    pub fn unbundle_files(&mut self) -> Vec<UnbundledFile> {
        let mut unbundled_files: Vec<UnbundledFile> = vec![];

        let mut inflated_stream = ByteStream::new(self.inflate_bundle());


        let file_count = inflated_stream.read_uint();
        let _unknown = inflated_stream.read(256);
        let _files = inflated_stream.read((16 * file_count) as usize);

        for _i in 0..file_count {
            let extension = inflated_stream.read_ulong();
            let path = inflated_stream.read_ulong();
            let has_data = inflated_stream.read_ulong();

            let data;
            if has_data > 0 {
                let _flag = inflated_stream.read_uint();
                let size = inflated_stream.read_uint();
                let _unknown2 = inflated_stream.read_uint();
                data = inflated_stream.read(size as usize);
            } else {
                data = vec![];
            }

            let unbundled_file = UnbundledFile {
                extension: extension,
                path: path,
                data: data,
            };

            unbundled_files.push(unbundled_file);
        }

        unbundled_files 
    }

    pub fn inflate_bundle(&mut self) -> Vec<u8> {
        let _header = self.compressed_stream.read_uint();
        let _size = self.compressed_stream.read_uint();
        let _reserved = self.compressed_stream.read_uint();

        assert_eq!(_header, 0xf0000004);
        assert_eq!(_reserved, 0);

        let mut result: Vec<u8> = vec![];

        while self.compressed_stream.remaining_bytes() > 0 {
            let len = self.compressed_stream.read_uint();
            let mut uncompressed_file = Vec::new();
            let mut decoder = ZlibDecoder::new(uncompressed_file);
            decoder.write_all(&self.compressed_stream.read(len as usize)).unwrap();
            uncompressed_file = decoder.finish().unwrap();
            result.append(&mut uncompressed_file);
        }
        result
    }
}