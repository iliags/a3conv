use std::{fs::File, io};

use lzss::{Lzss, SliceReader};

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

pub fn extract_archive(input_path: &str, _output_path: &str) -> Result<(), io::Error> {
    let file = File::open(input_path)?;

    type MyLzss = Lzss<13, 4, 0x20, { 1 << 10 }, { 2 << 10 }>;

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
