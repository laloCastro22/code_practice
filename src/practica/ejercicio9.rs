#[allow(dead_code)]
pub fn find_kth_bit(n: i32, k: i32) -> char {
    let mut flip: i32 = 0;
    let mut n = n;
    let mut k = k;

    while n > 1 {
        let mid = 1_i32 << (n - 1);
        if k == mid {
            return if flip == 1 { '0' } else { '1' };
        } else if k > mid {
            k = (1_i32 << n) - k;
            flip ^= 1;
        }

        n -= 1;
    }

    if flip == 1 { '1' } else { '0' }
}