#![allow(dead_code, unused_imports, unused_variables)]
use lzss::{Lzss, SliceReader, SliceWriter};
use std::{
    fs::File,
    io::{self, BufReader, Read, Seek, Write},
};

/* QuickBMS script
comtype lzss
endian big
get asize asize
do
getdstring name 13
get zsize long
get size long
savepos offset
clog name offset zsize size
math offset += zsize
goto offset
while offset < asize
*/

// TODO: Add a builder pattern implementation which defaults to the QuickBMS script
// TODO: Unify error handling

pub fn extract_archive(input_path: &String, output_path: &String) -> Result<(), io::Error> {
    // Open input file and get the size in bytes
    let file = File::open(&input_path)?;
    let asize = file.metadata().unwrap().len() as usize;

    // Create file reader
    let mut reader = BufReader::new(file);

    // Read archive size
    // Note: This cuts off the first 4 bytes of the archive, but the archive isn't a normal LZSS archive
    /*
    let mut asize = [0; 4];
    reader.read_exact(&mut asize)?;
    let asize = u32::from_be_bytes(asize) as usize;
    println!("Archive size: {:?}", asize);
     */

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

        //let seek = -8 as i64;
        //println!("Current position: {:?}", reader.stream_position()?);
        //reader.seek_relative(seek)?;
        //println!("Current position: {:?}", reader.stream_position()?);

        // Extract file
        let output_file = format!("{}/{}", output_path, name);
        extract_file(&mut reader, &output_file, zsize, size)?;

        offset += 13 + 4 + 4 + zsize;

        // Temporary break
        temp += 1;

        if temp > 0 {
            break;
        }
    }

    Ok(())
}

fn extract_file(
    reader: &mut BufReader<File>,
    output_file: &str,
    compressed_size: usize,
    uncompressed_size: usize,
) -> std::io::Result<()> {
    const EI: usize = 13;
    type A3Lzss = Lzss<EI, 4, 0x10, { 1 << EI }, { 2 << EI }>;

    let mut compressed_data = vec![0u8; compressed_size];
    reader.read_exact(&mut compressed_data)?;

    let mut decompressed_data = vec![0u8; uncompressed_size];
    match A3Lzss::decompress_stack(
        SliceReader::new(&compressed_data),
        SliceWriter::new(&mut decompressed_data),
    ) {
        Ok(_) => (),
        Err(e) => eprintln!("Extraction Error: {}", e),
    }

    let mut output_file = File::create(output_file)?;
    println!("Writing to file: {:?}", output_file);
    output_file.write_all(&decompressed_data)?;

    Ok(())
}

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
