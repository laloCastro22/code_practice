#[allow(dead_code)]
pub fn min_operations_11(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut mism_a = 0;
    let mut mism_b = 0;

    for (i, &ch) in bytes.iter().enumerate() {
        let expect_a = if i % 2 == 0 { b'0' } else { b'1' };
        let expect_b = if i % 2 == 0 { b'1' } else { b'0' };

        if ch != expect_a { mism_a += 1; }
        if ch != expect_b { mism_b += 1; }
    }

    mism_a.min(mism_b)
}