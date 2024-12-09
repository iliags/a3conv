// LZSS algorithm based on QuickBMS

pub fn unlzss(
    src_data: &Vec<u8>,
    src_size: usize,
    dest_data: &mut Vec<u8>,
    dest_size: usize,
) -> usize {
    let ei = 12;
    let ej = 4;
    let p = 2;
    let rless = p;
    let init_char = 0x20;

    let N = 1 << ei;
    let F = 1 << ej;

    let mut slide_window_size = N;

    return 0;
}
