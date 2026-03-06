#[allow(dead_code)]
pub fn check_ones_segment(s: String) -> bool {
        let mut bloque = 0;
        let mut count_ones = 0;
        let bytes_s = s.as_bytes();
        for &ch in bytes_s {
                if ch == b'1' {
                        count_ones += 1;
                }
                if ch == b'0' {
                        if count_ones > 0 {
                                bloque +=1;
                                count_ones = 0;
                        }
                }
        }
        if count_ones > 0 {
                bloque+=1;
        }
        bloque == 1
}
