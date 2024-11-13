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

#[cfg(test)]
mod tests {
    //use super::*;
}
