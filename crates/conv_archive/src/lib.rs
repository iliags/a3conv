#![allow(dead_code, unused_imports, unused_variables)]
use lzss::{Lzss, SliceReader, SliceWriter};
use std::{
    fs::File,
    io::{self, BufReader, Read},
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

pub fn extract_archive(input_path: &String, _output_path: &String) -> Result<(), io::Error> {
    // Open input file
    let file = File::open(&input_path)?;
    let mut reader = BufReader::new(file);

    // Read archive size
    let mut asize = [0; 4];
    reader.read_exact(&mut asize)?;
    let asize = u32::from_be_bytes(asize) as usize;

    // Create decompression instance
    const NAME_SIZE: usize = 13;
    type MyLzss = Lzss<NAME_SIZE, 4, 0x20, { 1 << NAME_SIZE }, { 2 << NAME_SIZE }>;

    //let mut output = [0; 30];
    //MyLzss::decompress_stack(SliceReader::new(&mut reader), output).unwrap();

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
