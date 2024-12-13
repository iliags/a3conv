#![allow(dead_code, unused_imports, unused_variables)]
//use lzss::{Lzss, SliceReader, SliceWriter};
use std::{
    fs::File,
    io::{self, BufReader, Read, Seek, Write},
};

use conv_codec::lzss::unlzss;

// TODO: Add a builder pattern implementation which defaults to the QuickBMS script
// TODO: Unify error handling

pub fn extract_archive(input_path: &String, output_path: &String) -> Result<(), io::Error> {
    // Open input file and get the size in bytes
    let file = File::open(&input_path)?;
    let asize = file.metadata().unwrap().len() as usize;

    //println!("Archive size: {:?}", asize);

    // Create file reader
    let mut reader = BufReader::new(file);
    let mut offset: usize = 0;
    let mut temp = 0;

    while offset < asize {
        // Read file name
        let mut name_bytes = [0; 13];
        reader.read_exact(&mut name_bytes)?;
        let name = String::from_utf8_lossy(&name_bytes)
            .trim_end_matches('\0')
            .to_string();

        println!("Extracting file: {:?}", name);

        // Read compressed size
        let mut zsize = [0; 4];
        reader.read_exact(&mut zsize)?;
        let zsize = u32::from_be_bytes(zsize) as usize;
        println!("Compressed size: {:?}", zsize);

        // Read uncompressed size
        let mut size = [0; 4];
        reader.read_exact(&mut size)?;
        let size = u32::from_be_bytes(size) as usize;
        println!("Uncompressed size: {:?}", size);

        // Extract file
        let output_file = format!("{}/{}", output_path, name);
        //extract_file(&mut reader, &output_file, zsize, size)?;

        /*******************************************************************/

        let mut compressed_data = vec![0u8; zsize];
        reader.read_exact(&mut compressed_data)?;
        //compressed_data.reverse(); // Reverse the bytes to little-endian

        println!("Compressed data: {:X?}", &compressed_data[..16]); // Print first 16 bytes of compressed data

        let mut decompressed_data = vec![0u8; size];

        match unlzss(&compressed_data, &mut decompressed_data) {
            Ok(_) => (),
            Err(e) => eprintln!("Extraction Error: {}", e),
        }

        //println!("Decompressed data: {:X?}", &decompressed_data[..16]); // Print first 16 bytes of decompressed data
        //println!("Target data: [A, 5, 1, 8, 0, 0, 0, 0, 23, 0, 1F, 0, 2C, 1, 2C, 1]");

        // Print second 16 bytes of decompressed data
        println!("Decompressed data: {:X?}", &decompressed_data[16..32]);

        let mut output_file = File::create(output_file)?;
        println!("Writing to file: {:?}", output_file);
        output_file.write_all(&decompressed_data)?;

        /*******************************************************************/

        //offset += 13 + 4 + 4 + zsize;
        offset += zsize;

        // Temporary break
        temp += 1;

        if temp > 0 {
            break;
        }
    }

    Ok(())
}

/*
#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_lzss() {
        use lzss::{Lzss, SliceReader, SliceWriter};
        type MyLzss = Lzss<10, 4, 0x20, { 1 << 10 }, { 2 << 10 }>;
        let input = b"Example Data";
        let mut output = [0; 30];
        let result = MyLzss::compress_stack(SliceReader::new(input), SliceWriter::new(&mut output));
        assert_eq!(result, Ok(14));
    }
}
 */
