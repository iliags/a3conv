#![allow(dead_code, unused_imports, unused_variables)]
use std::{
    fs::File,
    io::{self, BufReader, Read, Seek, Write},
};

pub mod lzss;
use lzss::unlzss;

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
    let mut file_count = 0;

    while offset < asize {
        // Read file name
        let mut name_bytes = [0; 13];
        match reader.read_exact(&mut name_bytes) {
            Ok(_) => (),
            Err(e) => {
                //eprintln!("Error reading file name: {}", e);
                //println!("Offset: {:?}", offset);
                //println!("Archive size: {:?}", asize);
                break;
            }
        }
        let name = String::from_utf8_lossy(&name_bytes)
            .trim_end_matches('\0')
            .to_string();

        //println!("Extracting file: {:?}", name);

        // Read compressed size
        let mut zsize = [0; 4];
        reader.read_exact(&mut zsize)?;
        let zsize = u32::from_be_bytes(zsize) as usize;

        // Read uncompressed size
        let mut size = [0; 4];
        reader.read_exact(&mut size)?;
        let size = u32::from_be_bytes(size) as usize;

        // Extract file
        let mut compressed_data = vec![0u8; zsize];
        reader.read_exact(&mut compressed_data)?;

        let mut decompressed_data = vec![0u8; size];

        match unlzss(&compressed_data, &mut decompressed_data) {
            Ok(_) => (),
            Err(e) => eprintln!("Extraction Error: {}", e),
        }

        let output_file = format!("{}/{}", output_path, name);
        let mut output_file = File::create(output_file)?;
        //println!("Writing to file: {:?}", output_file);
        output_file.write_all(&decompressed_data)?;
        file_count += 1;

        offset += zsize;

        // TODO: Remove this
        #[cfg(debug_assertions)]
        if file_count == -1 {
            break;
        }
    }

    println!("Extracted {} files", file_count);

    Ok(())
}
