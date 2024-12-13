/// LZSS algorithm based on QuickBMS LZSS implementation
///

// Note: The first few iterations are a direct translation of the C code, it will be refactored into idiomatic Rust later.
pub fn unlzss(src_data: &[u8], dest_data: &mut [u8]) -> Result<(), &'static str> {
    let ei = 12;
    let ej = 4;
    let p = 2;
    let rless = p;
    let init_char = 0x20;

    let mut n = 1 << ei;
    let mut f = 1 << ej;

    let mut slide_window = vec![0u8; n + f];
    let slide_window_size = n;
    lzss_set_window(&mut slide_window, slide_window_size, init_char);

    let mut r = (n - f) - rless;
    n -= 1;
    f -= 1;

    let mut flags: u32 = 0;
    let mut src_index = 0;
    let mut dest_index = 0;
    loop {
        flags >>= 1;

        if flags & 0x100 == 0 {
            if src_index >= src_data.len() {
                break;
            }
            flags = src_data[src_index] as u32 | 0xff00;

            src_index += 1;
        }
        if flags & 1 != 0 {
            if src_index >= src_data.len() {
                break;
            }
            let c = src_data[src_index] as u32;
            src_index += 1;

            if dest_index >= dest_data.len() {
                return Err("Error: dest_index out of bounds");
            }

            dest_data[dest_index] = c as u8;
            dest_index += 1;

            slide_window[r] = c as u8;
            r = (r + 1) & n;
        } else {
            if src_index >= src_data.len() {
                break;
            }
            let mut i = src_data[src_index] as u32;
            src_index += 1;

            if src_index >= src_data.len() {
                break;
            }
            let mut j = src_data[src_index] as u32;
            src_index += 1;

            i |= (j >> ej) << 8;
            j = (j & f as u32) + p as u32;

            for k in 0..=j {
                let win_index = ((i + k) as usize) & n;
                let c = slide_window[win_index] as u32;

                if dest_index >= dest_data.len() {
                    return Err("Error: dest_index out of bounds");
                }
                dest_data[dest_index] = c as u8;
                dest_index += 1;

                slide_window[r] = c as u8;
                r = (r + 1) & n;
            }
        }
    }

    Ok(())
}

fn lzss_set_window(window: &mut Vec<u8>, window_size: usize, init_chr: i32) {
    match init_chr {
        -1 => {
            window.fill(0);
            let mut i = 0;
            loop {
                let n = (i * 8) + 6;
                if n >= window_size {
                    break;
                }
                window[n] = i as u8;
                i += 1;
            }
        }
        -2 => {
            // invented
            for i in 0..window_size {
                window[i] = i as u8;
            }
        }
        -3 => {
            // invented
            for i in (0..window_size).rev() {
                window[i] = i as u8;
            }
        }
        _ => {
            window.fill(init_chr as u8);
        }
    }
}
